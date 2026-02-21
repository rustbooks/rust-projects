// Example utility to generate password hashes for initial admin user
// Run with: cargo run --example generate_hash

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

fn main() {
    println!("Password Hash Generator for Exam Automation System");
    println!("==================================================\n");

    // Get password from command line or use default
    let password = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "Admin@123".to_string());

    // Generate salt
    let salt = SaltString::generate(&mut OsRng);
    let salt_b64 = salt.to_string();

    // Hash password with salt
    let argon2 = Argon2::default();
    let salted_password = format!("{}{}", password, salt_b64);

    let password_hash = argon2
        .hash_password(salted_password.as_bytes(), &salt)
        .expect("Failed to hash password");

    println!("Password: {}", password);
    println!("Salt (Base64): {}", salt_b64);
    println!("Hash: {}\n", password_hash);

    println!("SQL to insert admin user:");
    println!("-------------------------");
    println!(
        r#"
INSERT INTO users (id, email, password_hash, salt, role, is_first_login, created_at, updated_at)
VALUES (
    uuid_generate_v4(),
    'admin@example.com',
    '{}',
    '{}',
    'admin',
    false,
    NOW(),
    NOW()
);
"#,
        password_hash, salt_b64
    );

    println!("\nUsage:");
    println!("------");
    println!("1. Copy the SQL statement above");
    println!("2. Run: psql -d exam_automation");
    println!("3. Paste and execute the SQL");
    println!("4. Login with email: admin@example.com, password: {}", password);
}
