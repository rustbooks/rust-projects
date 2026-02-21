use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub email: String,
    pub role: String,
    pub exp: i64, // Expiration timestamp
}

#[derive(Debug)]
pub struct PasswordHashResult {
    pub hash: String,
    pub salt: String,
}

/// Generate a new salt
pub fn generate_salt() -> String {
    SaltString::generate(&mut OsRng).to_string()
}

/// Hash password with a given salt using Argon2
pub fn hash_password_with_salt(password: &str, salt: &str) -> Result<PasswordHashResult, String> {
    let argon2 = Argon2::default();
    
    // Combine password with salt
    let salted_password = format!("{}{}", password, salt);
    
    let salt_string = SaltString::from_b64(salt)
        .map_err(|e| format!("Invalid salt: {}", e))?;
    
    let password_hash = argon2
        .hash_password(salted_password.as_bytes(), &salt_string)
        .map_err(|e| format!("Failed to hash password: {}", e))?;

    Ok(PasswordHashResult {
        hash: password_hash.to_string(),
        salt: salt.to_string(),
    })
}

/// Verify password against stored hash and salt
pub fn verify_password(password: &str, stored_hash: &str, salt: &str) -> Result<bool, String> {
    let argon2 = Argon2::default();
    
    // Combine password with salt
    let salted_password = format!("{}{}", password, salt);
    
    let parsed_hash = PasswordHash::new(stored_hash)
        .map_err(|e| format!("Failed to parse hash: {}", e))?;

    match argon2.verify_password(salted_password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Generate JWT token for authenticated user
pub fn generate_jwt(
    user_id: Uuid,
    email: &str,
    role: &str,
    secret: &str,
    expiration_hours: i64,
) -> Result<String, String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(expiration_hours))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        role: role.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| format!("Failed to generate JWT: {}", e))
}

/// Decode and validate JWT token
pub fn decode_jwt(token: &str, secret: &str) -> Result<Claims, String> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| format!("Invalid token: {}", e))
}

/// Generate random reset token
pub fn generate_reset_token() -> String {
    let mut rng = rand::thread_rng();
    let token: String = (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..62);
            let chars = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
            chars[idx] as char
        })
        .collect();
    token
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "TestPassword123!";
        let salt = generate_salt();
        
        let result = hash_password_with_salt(password, &salt).unwrap();
        
        assert!(verify_password(password, &result.hash, &salt).unwrap());
        assert!(!verify_password("WrongPassword", &result.hash, &salt).unwrap());
    }

    #[test]
    fn test_jwt_generation() {
        let user_id = Uuid::new_v4();
        let secret = "test-secret";
        
        let token = generate_jwt(user_id, "test@example.com", "user", secret, 24).unwrap();
        let claims = decode_jwt(&token, secret).unwrap();
        
        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.role, "user");
    }
}
