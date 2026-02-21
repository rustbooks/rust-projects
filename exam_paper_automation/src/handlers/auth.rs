use actix_web::{web, HttpResponse};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use validator::Validate;

use crate::models::users::{self, Entity as Users, UserRole};
use crate::utils::{
    auth::{generate_jwt, generate_reset_token, hash_password_with_salt, verify_password, generate_salt},
    email::EmailService,
};
use crate::config::Config;
use std::sync::Arc;
use crate::AppState;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub is_first_login: bool,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct VerifyTokenRequest {
    #[validate(email)]
    pub email: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    #[validate(email)]
    pub email: String,
    pub token: Option<String>, // For reset flow
    pub old_password: Option<String>, // For first login flow
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub new_password: String,
}

/// POST /api/auth/login
pub async fn login(
    data: web::Data<Arc<AppState>>,
    request: web::Json<LoginRequest>,
) -> HttpResponse {
    // Validate request
    if let Err(e) = request.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("Validation error: {}", e),
        });
    }

    let db = &data.db;

    // Find user by email
    let user = match Users::find()
        .filter(users::Column::Email.eq(&request.email))
        .one(db)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Invalid credentials".to_string(),
            });
        }
        Err(e) => {
            tracing::error!("Database error: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            });
        }
    };

    // Verify password
    match verify_password(&request.password, &user.password_hash, &user.salt) {
        Ok(true) => {}
        Ok(false) => {
            return HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Invalid credentials".to_string(),
            });
        }
        Err(e) => {
            tracing::error!("Password verification error: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            });
        }
    }

    // Generate JWT token
    let role_str = match user.role {
        UserRole::Admin => "admin",
        UserRole::User => "user",
    };

    let token = match generate_jwt(
        user.id,
        &user.email,
        role_str,
        &data.config.jwt_secret,
        data.config.jwt_expiration_hours,
    ) {
        Ok(token) => token,
        Err(e) => {
            tracing::error!("JWT generation error: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to generate token".to_string(),
            });
        }
    };

    HttpResponse::Ok().json(LoginResponse {
        token,
        is_first_login: user.is_first_login,
        role: role_str.to_string(),
    })
}

/// POST /api/auth/reset-password
pub async fn reset_password(
    data: web::Data<Arc<AppState>>,
    request: web::Json<ResetPasswordRequest>,
) -> HttpResponse {
    if let Err(e) = request.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("Validation error: {}", e),
        });
    }

    let db = &data.db;

    // Find user by email
    let user = match Users::find()
        .filter(users::Column::Email.eq(&request.email))
        .one(db)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            // Don't reveal if email exists
            return HttpResponse::Ok().json(serde_json::json!({
                "message": "If the email exists, a reset token has been sent"
            }));
        }
        Err(e) => {
            tracing::error!("Database error: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            });
        }
    };

    // Generate reset token
    let reset_token = generate_reset_token();
    let expiry = Utc::now() + Duration::hours(1);

    // Update user with reset token
    let mut user: users::ActiveModel = user.into();
    user.reset_token = Set(Some(reset_token.clone()));
    user.reset_token_expiry = Set(Some(expiry.into()));

    if let Err(e) = user.update(db).await {
        tracing::error!("Failed to update user: {}", e);
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to generate reset token".to_string(),
        });
    }

    // Send email with reset token
    let email_service = EmailService::new(
        data.config.smtp_host.clone(),
        data.config.smtp_port,
        data.config.smtp_username.clone(),
        data.config.smtp_password.clone(),
        data.config.from_email.clone(),
    );

    if let Err(e) = email_service
        .send_password_reset_email(&request.email, &reset_token)
        .await
    {
        tracing::error!("Failed to send email: {}", e);
        // Don't expose email sending failure to user
    }

    HttpResponse::Ok().json(serde_json::json!({
        "message": "If the email exists, a reset token has been sent"
    }))
}

/// POST /api/auth/verify-token
pub async fn verify_reset_token(
    data: web::Data<Arc<AppState>>,
    request: web::Json<VerifyTokenRequest>,
) -> HttpResponse {
    if let Err(e) = request.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("Validation error: {}", e),
        });
    }

    let db = &data.db;

    // Find user and verify token
    let user = match Users::find()
        .filter(users::Column::Email.eq(&request.email))
        .filter(users::Column::ResetToken.eq(&request.token))
        .one(db)
        .await
    {
        Ok(Some(user)) => user,
        _ => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid or expired token".to_string(),
            });
        }
    };

    // Check if token is expired
    if let Some(expiry) = user.reset_token_expiry {
        if Utc::now() > expiry.naive_utc().and_utc() {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Token has expired".to_string(),
            });
        }
    }

    HttpResponse::Ok().json(serde_json::json!({
        "message": "Token is valid"
    }))
}

/// POST /api/auth/change-password
pub async fn change_password(
    data: web::Data<Arc<AppState>>,
    request: web::Json<ChangePasswordRequest>,
) -> HttpResponse {
    if let Err(e) = request.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("Validation error: {}", e),
        });
    }

    let db = &data.db;

    // Find user
    let user = match Users::find()
        .filter(users::Column::Email.eq(&request.email))
        .one(db)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::NotFound().json(ErrorResponse {
                error: "User not found".to_string(),
            });
        }
        Err(e) => {
            tracing::error!("Database error: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            });
        }
    };

    // Verify authorization (either valid reset token or correct old password)
    let mut is_authorized = false;

    if let Some(token) = &request.token {
        // Reset token flow
        if let Some(stored_token) = &user.reset_token {
            if token == stored_token {
                if let Some(expiry) = user.reset_token_expiry {
                    if Utc::now() <= expiry.naive_utc().and_utc() {
                        is_authorized = true;
                    }
                }
            }
        }
    } else if let Some(old_password) = &request.old_password {
        // First login or regular password change flow
        match verify_password(old_password, &user.password_hash, &user.salt) {
            Ok(true) => is_authorized = true,
            _ => {}
        }
    }

    if !is_authorized {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Unauthorized to change password".to_string(),
        });
    }

    // Generate new salt and hash password
    let new_salt = generate_salt();
    let hash_result = match hash_password_with_salt(&request.new_password, &new_salt) {
        Ok(result) => result,
        Err(e) => {
            tracing::error!("Password hashing error: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to hash password".to_string(),
            });
        }
    };

    // Update user password and clear reset token
    let mut user: users::ActiveModel = user.into();
    user.password_hash = Set(hash_result.hash);
    user.salt = Set(new_salt);
    user.reset_token = Set(None);
    user.reset_token_expiry = Set(None);
    user.is_first_login = Set(false);

    if let Err(e) = user.update(db).await {
        tracing::error!("Failed to update user: {}", e);
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to update password".to_string(),
        });
        }

    HttpResponse::Ok().json(serde_json::json!({
        "message": "Password changed successfully"
    }))
}
