
# Exam Paper Automation System

# 1. Install dependencies
sudo apt-get update
sudo apt-get install -y build-essential libssl-dev pkg-config postgresql postgresql-contrib

# 2. Set up PostgreSQL
sudo systemctl start postgresql
sudo systemctl enable postgresql
sudo -u postgres createdb exam_automation
sudo -u postgres psql -c "CREATE USER examuser WITH PASSWORD 'exampass123';"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE exam_automation TO examuser;"

# 3. Navigate to project
cd exam_paper_automation

# 4. Apply schema
sudo -u postgres psql exam_automation -f schema.sql

# 5. Configure environment
cp .env.example .env
# Edit .env with your settings
nano .env

# 6. Generate admin password and create user
cargo run --example generate_hash Admin@123
# Copy the SQL output and run it:
# sudo -u postgres psql exam_automation
# Then paste the INSERT statement

# 7. Build and run
cargo build --release
cargo run --release

A comprehensive exam paper generation and management system built with Rust, PostgreSQL 17, and modern web technologies.

## 🏗️ Architecture

### Technology Stack
- **Backend**: Rust 1.93.0 with Actix-web 4.9
- **Database**: PostgreSQL 17
- **ORM**: SeaORM 1.1 (async, type-safe)
- **Authentication**: Salt-based Argon2 hashing + JWT
- **PDF Generation**: printpdf
- **Email**: Lettre for SMTP

### Key Features
1. **User Management**
   - Salt-based authentication with Argon2
   - JWT token-based authorization
   - Email-based password reset
   - Mandatory password change on first login

2. **Question Bank Management**
   - 4-mark, 12-mark, and MCQ questions
   - Unit-wise categorization
   - Automatic Bloom taxonomy assignment
   - Course Outcome (CO) and Program Outcome (PO) mapping
   - Duplicate detection with warnings
   - Full edit history tracking

3. **Admin Features**
   - Course and syllabus management
   - Paper format templates
   - Automatic paper generation with PDF export
   - Course Articulation Matrix (CAM) generation

## 📊 Database Schema

### Core Tables

#### Users & Authentication
```sql
users
├── id (UUID, PK)
├── email (UNIQUE)
├── password_hash (Argon2)
├── salt (Per-user salt)
├── role (admin/user)
├── is_first_login (BOOLEAN)
├── reset_token
└── reset_token_expiry
```

#### Course Management
```sql
courses
├── id (UUID, PK)
├── course_code (UNIQUE)
├── course_name
├── total_units
└── is_active

course_units
├── id (UUID, PK)
├── course_id (FK)
├── unit_number
├── unit_name
└── unit_content

course_outcomes (CO)
├── id (UUID, PK)
├── course_id (FK)
├── co_number
├── co_description
└── bloom_level
```

#### Question Bank
```sql
questions
├── id (UUID, PK)
├── course_id (FK)
├── unit_id (FK)
├── question_type (MCQ/4_MARK/12_MARK)
├── question_text (Full-text searchable)
├── marks
├── bloom_level (Remember/Understand/Apply/Analyze/Evaluate/Create)
├── co_id (FK)
├── difficulty_level
├── created_by (FK to users)
├── created_at
└── updated_at

mcq_options
├── id (UUID, PK)
├── question_id (FK)
├── option_text
├── is_correct
└── option_order
```

#### Paper Generation
```sql
paper_formats
├── id (UUID, PK)
├── course_id (FK)
├── format_name
├── header_template
├── footer_template
├── section_structure (JSONB)
├── total_marks
├── mcq_count
├── four_mark_count
└── twelve_mark_count

generated_papers
├── id (UUID, PK)
├── course_id (FK)
├── paper_format_id (FK)
├── paper_content (JSONB - question IDs)
├── pdf_path
└── generated_at
```

#### Audit & Tracking
```sql
question_history
├── id (UUID, PK)
├── question_id (FK)
├── action (CREATED/UPDATED/DELETED)
├── old_data (JSONB)
├── new_data (JSONB)
├── changed_by (FK)
└── changed_at
```

## 🚀 Installation & Setup

### Prerequisites
- Rust 1.93.0 or later
- PostgreSQL 17
- SMTP server access (for email functionality)

### Step 1: Database Setup
```bash
# Create database
createdb exam_automation

# Run schema
psql -d exam_automation -f schema.sql
```

### Step 2: Environment Configuration
```bash
# Copy example environment file
cp .env.example .env

# Edit .env with your configuration
nano .env
```

Required environment variables:
```env
DATABASE_URL=postgresql://user:password@localhost:5432/exam_automation
JWT_SECRET=your-secret-key
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password
FROM_EMAIL=noreply@examautomation.com
```

### Step 3: Build and Run
```bash
# Build the project
cargo build --release

# Run the server
cargo run --release
```

The server will start at `http://127.0.0.1:8080`

## 📡 API Endpoints

### Authentication (`/api/auth`)
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/login` | Login with email and password |
| POST | `/reset-password` | Request password reset token |
| POST | `/verify-token` | Verify reset token validity |
| POST | `/change-password` | Change password (first login or reset) |

### User Routes (`/api/user`) - Requires Authentication
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/questions/4-mark` | Add 4-mark question |
| POST | `/questions/12-mark` | Add 12-mark question |
| POST | `/questions/mcq` | Add MCQ question |
| GET | `/questions/search` | Search questions (by course, unit, type) |
| GET | `/questions/history` | Get user's question history |
| PUT | `/questions/{id}` | Edit existing question |

### Admin Routes (`/api/admin`) - Requires Admin Role
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/questions/search` | Search all questions |
| POST | `/paper-format` | Upload paper format template |
| GET | `/paper-format/{course_id}` | Get paper format |
| PUT | `/paper-format/{id}` | Update paper format |
| GET | `/courses` | List all courses |
| POST | `/courses` | Add new course |
| POST | `/courses/{id}/units` | Add course units |
| POST | `/generate-paper/{course_id}` | Generate exam paper PDF |

## 🔐 Authentication Flow

### 1. First Login
```
1. Admin creates user account with temporary password
2. Email sent to user with credentials
3. User logs in with temporary password
4. is_first_login = true → forced password change
5. User submits new password
6. Account activated
```

### 2. Password Reset
```
1. User requests password reset
2. System generates secure token
3. Token emailed to user (expires in 1 hour)
4. User submits token + new password
5. Password updated, token cleared
```

### 3. JWT Token
```
1. User logs in successfully
2. Server generates JWT with:
   - User ID
   - Email
   - Role (admin/user)
   - Expiration (24 hours default)
3. Client includes token in Authorization header:
   Authorization: Bearer <token>
```

## 📝 Question Management

### Adding Questions

**4-Mark Question:**
```json
POST /api/user/questions/4-mark
{
  "course_code": "CS101",
  "unit_number": 1,
  "question_text": "Explain the concept of polymorphism in OOP",
  "difficulty_level": "Medium"
}
```

**MCQ Question:**
```json
POST /api/user/questions/mcq
{
  "course_code": "CS101",
  "unit_number": 2,
  "question_text": "Which of the following is NOT a primitive data type?",
  "options": [
    {"text": "int", "is_correct": false},
    {"text": "String", "is_correct": true},
    {"text": "boolean", "is_correct": false},
    {"text": "double", "is_correct": false}
  ]
}
```

### Automatic Assignment
When a question is added:
1. **Duplicate Check**: System searches for similar questions
2. **Bloom Level**: Automatically assigned based on course syllabus
3. **CO Mapping**: Linked to appropriate Course Outcome
4. **PO Mapping**: Automatically mapped via CO-PO matrix
5. **History**: Logged with timestamp and user

### Search & Filter
```
GET /api/user/questions/search?course_code=CS101&unit=1&type=4_MARK
```

## 📄 Paper Generation

### 1. Upload Format Template (Admin)
```json
POST /api/admin/paper-format
{
  "course_id": "...",
  "format_name": "Mid-Semester 2024",
  "total_marks": 100,
  "mcq_count": 20,
  "four_mark_count": 5,
  "twelve_mark_count": 3,
  "section_structure": {
    "sections": [
      {
        "name": "Section A - MCQ",
        "type": "MCQ",
        "count": 20,
        "marks_each": 1
      },
      {
        "name": "Section B - Short Answer",
        "type": "4_MARK",
        "count": 5,
        "marks_each": 4
      }
    ]
  }
}
```

### 2. Generate Paper
```
POST /api/admin/generate-paper/{course_id}
```

System will:
1. Select questions based on format
2. Ensure bloom level distribution
3. Calculate CO-PO matrix
4. Generate PDF with:
   - Header/Footer from template
   - Questions organized by section
   - Course Articulation Matrix table
   - Marks distribution

## 🎯 Course Articulation Matrix (CAM)

The system automatically generates CAM showing:
- CO vs PO mapping
- Bloom taxonomy distribution
- Marks allocation per CO
- Assessment coverage

Example output:
```
| CO | Questions | Bloom | PO1 | PO2 | PO3 | Total Marks |
|----|-----------|-------|-----|-----|-----|-------------|
| CO1| Q1, Q5    | Apply |  3  |  2  |  1  |     16      |
| CO2| Q2, Q6    | Analyze| 2  |  3  |  2  |     16      |
```

## 🔧 Development

### Project Structure
```
exam_paper_automation/
├── src/
│   ├── main.rs              # Entry point
│   ├── config.rs            # Configuration management
│   ├── models/              # SeaORM entities
│   │   ├── users.rs
│   │   ├── questions.rs
│   │   └── ...
│   ├── handlers/            # HTTP request handlers
│   │   ├── auth.rs
│   │   ├── user.rs
│   │   └── admin.rs
│   ├── services/            # Business logic
│   │   ├── question.rs
│   │   ├── paper_gen.rs
│   │   └── cam.rs
│   ├── middleware/          # Authentication middleware
│   │   └── auth.rs
│   └── utils/               # Utilities
│       ├── auth.rs          # Password hashing, JWT
│       ├── email.rs         # Email sending
│       ├── validation.rs    # Input validation
│       └── pdf.rs           # PDF generation
├── Cargo.toml
├── .env.example
└── schema.sql
```

### Running Tests
```bash
cargo test
```

### Database Migrations
```bash
# Create new migration
sea-orm-cli migrate generate create_new_table

# Run migrations
sea-orm-cli migrate up
```

## 📊 Performance Considerations

1. **Indexing**: Full-text search on question_text using GIN index
2. **Caching**: Consider Redis for frequently accessed course data
3. **Connection Pooling**: SeaORM handles connection pooling automatically
4. **Async Operations**: All database operations are async for better concurrency

## 🔒 Security Features

1. **Password Security**
   - Argon2 hashing (memory-hard, resistant to GPU attacks)
   - Per-user salt (prevents rainbow table attacks)
   - Mandatory password change on first login

2. **Token Security**
   - JWT with expiration
   - Secure random token generation for password reset
   - Token expiry (1 hour for reset tokens)

3. **Input Validation**
   - Email validation
   - Password strength requirements (min 8 chars)
   - SQL injection prevention via ORM

4. **Authorization**
   - Role-based access control (Admin/User)
   - Middleware for protected routes
   - User can only edit own questions

## 📈 Future Enhancements

1. **Advanced Features**
   - Question bank import/export (CSV, Excel)
   - Question versioning
   - Collaborative editing
   - Question difficulty prediction using ML

2. **Analytics**
   - Question usage statistics
   - Bloom level distribution charts
   - Student performance correlation

3. **Integration**
   - LMS integration (Moodle, Canvas)
   - Cloud storage for PDFs (S3, GCS)
   - Real-time collaboration

## 📞 Support

For issues or questions:
1. Check the logs: `RUST_LOG=debug cargo run`
2. Verify database connectivity
3. Check SMTP configuration for email issues

## 📝 License

This project is built for educational purposes.

---

**Built with ❤️ using Rust and PostgreSQL**
