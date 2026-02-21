use actix_web::{web, HttpRequest, HttpResponse};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, 
    QueryFilter, QueryOrder, Set, PaginatorTrait
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::models::{
    questions::{self, Entity as Questions, QuestionType, BloomLevel},
    mcq_options::{self, Entity as McqOptions},
    courses::{self, Entity as Courses},
    course_units::{self, Entity as CourseUnits},
};
use crate::utils::auth::Claims;

#[derive(Debug, Deserialize, Validate)]
pub struct Add4MarkQuestionRequest {
    pub course_code: String,
    pub unit_number: i32,
    #[validate(length(min = 10, message = "Question text must be at least 10 characters"))]
    pub question_text: String,
    pub difficulty_level: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct Add12MarkQuestionRequest {
    pub course_code: String,
    pub unit_number: i32,
    #[validate(length(min = 10, message = "Question text must be at least 10 characters"))]
    pub question_text: String,
    pub difficulty_level: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct McqOption {
    pub text: String,
    pub is_correct: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddMcqQuestionRequest {
    pub course_code: String,
    pub unit_number: i32,
    #[validate(length(min = 10, message = "Question text must be at least 10 characters"))]
    pub question_text: String,
    #[validate(length(min = 2, max = 6, message = "MCQ must have 2-6 options"))]
    pub options: Vec<McqOption>,
}

#[derive(Debug, Serialize)]
pub struct QuestionResponse {
    pub id: Uuid,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// Helper function to extract user claims from request
fn get_user_claims(req: &HttpRequest) -> Result<Claims, String> {
    req.extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| "Unauthorized".to_string())
}

// Helper function to check for duplicate questions
async fn check_duplicate_question(
    db: &DatabaseConnection,
    course_id: Uuid,
    question_text: &str,
) -> Result<Option<questions::Model>, String> {
    // Use full-text search or simple LIKE for duplicate detection
    Questions::find()
        .filter(questions::Column::CourseId.eq(course_id))
        .filter(questions::Column::QuestionText.contains(question_text))
        .one(db)
        .await
        .map_err(|e| format!("Database error: {}", e))
}

/// POST /api/user/questions/4-mark
pub async fn add_4_mark_question(
    req: HttpRequest,
    data: web::Data<std::sync::Arc<crate::AppState>>,
    request: web::Json<Add4MarkQuestionRequest>,
) -> HttpResponse {
    // Validate request
    if let Err(e) = request.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("Validation error: {}", e),
        });
    }

    // Get user from token
    let claims = match get_user_claims(&req) {
        Ok(claims) => claims,
        Err(e) => return HttpResponse::Unauthorized().json(ErrorResponse { error: e }),
    };

    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Invalid user ID".to_string(),
            })
        }
    };

    let db = &data.db;

    // Find course by code
    let course = match Courses::find()
        .filter(courses::Column::CourseCode.eq(&request.course_code))
        .one(db)
        .await
    {
        Ok(Some(course)) => course,
        Ok(None) => {
            return HttpResponse::NotFound().json(ErrorResponse {
                error: format!("Course with code '{}' not found", request.course_code),
            })
        }
        Err(e) => {
            tracing::error!("Database error: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            });
        }
    };

    // Find unit
    let unit = match CourseUnits::find()
        .filter(course_units::Column::CourseId.eq(course.id))
        .filter(course_units::Column::UnitNumber.eq(request.unit_number))
        .one(db)
        .await
    {
        Ok(Some(unit)) => unit,
        Ok(None) => {
            return HttpResponse::NotFound().json(ErrorResponse {
                error: format!("Unit {} not found for course {}", request.unit_number, request.course_code),
            })
        }
        Err(e) => {
            tracing::error!("Database error: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            });
        }
    };

    // Check for duplicate
    match check_duplicate_question(db, course.id, &request.question_text).await {
        Ok(Some(existing)) => {
            return HttpResponse::Conflict().json(serde_json::json!({
                "error": "A similar question already exists",
                "existing_question_id": existing.id,
                "existing_question": existing.question_text,
                "message": "Please modify your question or use the existing one"
            }));
        }
        Ok(None) => {},
        Err(e) => {
            tracing::error!("Duplicate check error: {}", e);
            // Continue anyway - better to allow potential duplicate than fail
        }
    }

    // TODO: Automatically determine Bloom level and CO based on syllabus
    // For now, using default values
    let bloom_level = BloomLevel::Understand;

    // Create question
    let new_question = questions::ActiveModel {
        id: Set(Uuid::new_v4()),
        course_id: Set(course.id),
        unit_id: Set(unit.id),
        question_type: Set(QuestionType::FourMark),
        question_text: Set(request.question_text.clone()),
        marks: Set(4),
        bloom_level: Set(bloom_level),
        co_id: Set(None), // TODO: Auto-assign based on unit and bloom level
        difficulty_level: Set(request.difficulty_level.clone().map(|d| {
            match d.as_str() {
                "Easy" => questions::DifficultyLevel::Easy,
                "Hard" => questions::DifficultyLevel::Hard,
                _ => questions::DifficultyLevel::Medium,
            }
        })),
        created_by: Set(user_id),
        created_at: Set(chrono::Utc::now().into()),
        updated_at: Set(chrono::Utc::now().into()),
    };

    match new_question.insert(db).await {
        Ok(question) => HttpResponse::Created().json(QuestionResponse {
            id: question.id,
            message: "4-mark question added successfully".to_string(),
        }),
        Err(e) => {
            tracing::error!("Failed to insert question: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to add question".to_string(),
            })
        }
    }
}

/// POST /api/user/questions/12-mark
pub async fn add_12_mark_question(
    req: HttpRequest,
    data: web::Data<std::sync::Arc<crate::AppState>>,
    request: web::Json<Add12MarkQuestionRequest>,
) -> HttpResponse {
    // Similar to add_4_mark_question but with marks = 12
    // Implementation follows same pattern
    HttpResponse::NotImplemented().json(ErrorResponse {
        error: "Implementation similar to 4-mark question".to_string(),
    })
}

/// POST /api/user/questions/mcq
pub async fn add_mcq_question(
    req: HttpRequest,
    data: web::Data<std::sync::Arc<crate::AppState>>,
    request: web::Json<AddMcqQuestionRequest>,
) -> HttpResponse {
    // Implementation similar to add_4_mark_question
    // Additionally create mcq_options entries
    HttpResponse::NotImplemented().json(ErrorResponse {
        error: "Implementation includes MCQ options creation".to_string(),
    })
}

/// GET /api/user/questions/search
pub async fn search_questions(
    req: HttpRequest,
    data: web::Data<std::sync::Arc<crate::AppState>>,
    query: web::Query<serde_json::Value>,
) -> HttpResponse {
    HttpResponse::NotImplemented().json(ErrorResponse {
        error: "Search implementation with filters".to_string(),
    })
}

/// GET /api/user/questions/history
pub async fn get_question_history(
    req: HttpRequest,
    data: web::Data<std::sync::Arc<crate::AppState>>,
) -> HttpResponse {
    let claims = match get_user_claims(&req) {
        Ok(claims) => claims,
        Err(e) => return HttpResponse::Unauthorized().json(ErrorResponse { error: e }),
    };

    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Invalid user ID".to_string(),
            })
        }
    };

    // Query using the view v_user_question_history
    // Return questions with formatted dates
    HttpResponse::NotImplemented().json(ErrorResponse {
        error: "History retrieval from database view".to_string(),
    })
}

/// PUT /api/user/questions/{id}
pub async fn update_question(
    req: HttpRequest,
    data: web::Data<std::sync::Arc<crate::AppState>>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    HttpResponse::NotImplemented().json(ErrorResponse {
        error: "Update question implementation".to_string(),
    })
}
