# Quick Start Guide - Exam Paper Automation System

## 🚀 Get Started in 5 Minutes

### Prerequisites
```bash
# Check installations
rustc --version  # Should be 1.93.0+
psql --version   # Should be PostgreSQL 17+
```

### Step 1: Database Setup (2 minutes)
```bash
# Create database
createdb exam_automation

# Apply schema
cd exam_paper_automation
psql -d exam_automation -f schema.sql
```

### Step 2: Configure Environment (1 minute)
```bash
# Copy example config
cp .env.example .env

# Edit with your details (use nano or your preferred editor)
nano .env
```

**Minimum required changes**:
```env
DATABASE_URL=postgresql://YOUR_USER:YOUR_PASSWORD@localhost:5432/exam_automation
JWT_SECRET=CHANGE_THIS_TO_RANDOM_STRING_32_CHARS
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password
```

### Step 3: Build & Run (2 minutes)
```bash
# Build (first time takes longer)
cargo build --release

# Run server
cargo run --release
```

Server starts at: `http://127.0.0.1:8080`

## 🧪 Test Your Setup

### 1. Create Admin User
```bash
psql -d exam_automation
```

```sql
-- Generate proper hash first with this Rust code:
-- cargo run --example generate_hash

INSERT INTO users (id, email, password_hash, salt, role, is_first_login)
VALUES (
    uuid_generate_v4(),
    'admin@test.com',
    '$argon2id$v=19$m=19456,t=2,p=1$YOUR_HASH_HERE',
    'YOUR_SALT_HERE',
    'admin',
    false
);
```

### 2. Test Login
```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@test.com",
    "password": "YourPassword123"
  }'
```

Response should contain:
```json
{
  "token": "eyJ0eXAi...",
  "is_first_login": false,
  "role": "admin"
}
```

### 3. Create a Course
```bash
# Save the token from login
TOKEN="YOUR_JWT_TOKEN_HERE"

curl -X POST http://localhost:8080/api/admin/courses \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "course_code": "CS101",
    "course_name": "Introduction to Programming",
    "total_units": 5
  }'
```

## 📚 Next Steps

1. **Read the README.md** for complete architecture
2. **Check IMPLEMENTATION_GUIDE.md** for detailed completion steps
3. **Explore the API** using the endpoint documentation

## 🔧 Common Commands

```bash
# Run in development mode with logging
RUST_LOG=debug cargo run

# Run tests
cargo test

# Format code
cargo fmt

# Check for issues
cargo clippy

# Build optimized release
cargo build --release

# Clean build artifacts
cargo clean
```

## 📊 Project Structure Overview

```
exam_paper_automation/
├── schema.sql                 # Database schema
├── Cargo.toml                 # Dependencies
├── README.md                  # Full documentation
├── IMPLEMENTATION_GUIDE.md    # Step-by-step completion
├── .env.example               # Configuration template
└── src/
    ├── main.rs               # Application entry point
    ├── config.rs             # Configuration
    ├── models/               # Database entities
    ├── handlers/             # HTTP request handlers
    ├── services/             # Business logic
    ├── middleware/           # Auth middleware
    └── utils/                # Utilities
```

## ⚡ Quick Tips

1. **Generate Secure JWT Secret**:
   ```bash
   openssl rand -hex 32
   ```

2. **Check Logs**:
   ```bash
   RUST_LOG=info cargo run 2>&1 | tee app.log
   ```

3. **Database Issues**:
   ```bash
   # Restart PostgreSQL
   sudo systemctl restart postgresql
   
   # Check connection
   psql -d exam_automation -c "SELECT version();"
   ```

4. **For Gmail SMTP**:
   - Enable 2-Factor Authentication
   - Generate App Password: https://myaccount.google.com/apppasswords
   - Use App Password in SMTP_PASSWORD

## 🎯 Current Status

✅ **Completed**:
- Database schema with all tables
- Authentication system (login, password reset)
- User entity and question models
- 4-mark question creation
- Middleware for auth and admin access

🚧 **To Complete** (See IMPLEMENTATION_GUIDE.md):
- 12-mark and MCQ question handlers
- Question search and history
- Admin course management
- Paper format and generation
- PDF creation with CAM

## 📞 Need Help?

1. Check the logs: `RUST_LOG=debug cargo run`
2. Verify database connectivity
3. Review IMPLEMENTATION_GUIDE.md for detailed steps
4. Check README.md for architecture details

---

**Happy Coding! 🦀**
