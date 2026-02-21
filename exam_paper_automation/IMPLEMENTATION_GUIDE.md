# Implementation Guide - Exam Paper Automation System

## 🎯 Overview

This guide provides step-by-step instructions to complete the exam paper automation system. The foundation has been laid with the PostgreSQL schema, Rust project structure, authentication system, and core handlers.

## ✅ What's Already Implemented

### 1. Database Schema (`schema.sql`)
- ✅ Complete PostgreSQL 17 schema
- ✅ Users table with salt-based authentication
- ✅ Courses, units, and syllabus tables
- ✅ Questions table with full-text search
- ✅ Course Outcomes (CO) and Program Outcomes (PO)
- ✅ CO-PO mapping tables
- ✅ Paper formats and generation tracking
- ✅ Question history audit trail
- ✅ Triggers for automatic timestamps and logging
- ✅ Views for reporting (v_user_question_history, v_course_articulation_report)

### 2. Rust Project Structure
- ✅ Cargo.toml with latest dependencies (Rust 1.93.0)
- ✅ Main application entry point with Actix-web
- ✅ Configuration management (.env support)
- ✅ SeaORM entity models (users, questions)
- ✅ Authentication utilities (Argon2 + salt, JWT)
- ✅ Email service for password reset
- ✅ Authentication middleware

### 3. Core Features
- ✅ User login with JWT tokens
- ✅ Password reset with email tokens
- ✅ First-login password change flow
- ✅ Role-based access control (Admin/User middleware)
- ✅ 4-mark question addition with duplicate detection
- ✅ API routing structure

## 🚧 What Needs to Be Completed

### Phase 1: Complete Question Management (User Routes)

#### 1.1 Complete 12-Mark and MCQ Handlers
**File**: `src/handlers/user.rs`

**Tasks**:
- [ ] Implement `add_12_mark_question` (similar to 4-mark but marks=12)
- [ ] Implement `add_mcq_question`:
  - Create question entry
  - Create 2-6 MCQ options
  - Validate exactly one correct answer
  - Transaction to ensure atomic creation

**Example Implementation**:
```rust
pub async fn add_mcq_question(...) -> HttpResponse {
    // 1. Validate: must have 2-6 options, exactly one correct
    let correct_count = request.options.iter().filter(|o| o.is_correct).count();
    if correct_count != 1 {
        return HttpResponse::BadRequest()...;
    }
    
    // 2. Begin transaction
    let txn = db.begin().await?;
    
    // 3. Create question
    let question_id = create_question(...);
    
    // 4. Create options
    for (index, option) in request.options.iter().enumerate() {
        create_mcq_option(question_id, option, index);
    }
    
    // 5. Commit transaction
    txn.commit().await?;
}
```

#### 1.2 Implement Question Search
**File**: `src/handlers/user.rs`

**Features**:
- Search by course code
- Filter by unit number
- Filter by question type (MCQ/4_MARK/12_MARK)
- Sort by created date
- Pagination support

**Query Parameters**:
```
GET /api/user/questions/search?course_code=CS101&unit=1&type=4_MARK&page=1&limit=20
```

**Implementation**:
```rust
pub async fn search_questions(...) -> HttpResponse {
    let course_code = query.get("course_code");
    let unit = query.get("unit");
    let q_type = query.get("type");
    
    let mut query = Questions::find();
    
    if let Some(code) = course_code {
        // Join with courses table and filter
        query = query.filter(...);
    }
    
    if let Some(unit_num) = unit {
        query = query.filter(questions::Column::UnitId.eq(...));
    }
    
    // Execute with pagination
    let results = query.paginate(db, 20).fetch_page(page).await?;
}
```

#### 1.3 Implement Question History
**File**: `src/handlers/user.rs`

**Tasks**:
- [ ] Query `v_user_question_history` view
- [ ] Filter by current user ID
- [ ] Format dates as DD-MM-YYYY
- [ ] Format time as HH:MM:SS
- [ ] Include edit button data for frontend

**SQL Query to Use**:
```sql
SELECT * FROM v_user_question_history 
WHERE created_by_email = $1 
ORDER BY created_at DESC
```

#### 1.4 Implement Question Update
**File**: `src/handlers/user.rs`

**Tasks**:
- [ ] Verify user owns the question
- [ ] Update question fields
- [ ] For MCQ: handle option updates
- [ ] Log changes to question_history table (via trigger)

### Phase 2: Complete Entity Models

**Files**: `src/models/*.rs`

**Tasks**:
- [ ] Complete `course_units.rs`
- [ ] Complete `course_outcomes.rs`
- [ ] Complete `program_outcomes.rs`
- [ ] Complete `mcq_options.rs`
- [ ] Complete `paper_formats.rs`
- [ ] Complete `generated_papers.rs`
- [ ] Complete `question_history.rs`

**Template for Each**:
```rust
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "table_name")]
pub struct Model {
    // Map all columns from schema.sql
    // Include proper types and constraints
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // Define foreign key relationships
}

impl ActiveModelBehavior for ActiveModel {}
```

### Phase 3: Implement Admin Features

#### 3.1 Course Management
**File**: `src/handlers/admin.rs`

**Tasks**:
- [ ] `list_courses`: Return all courses with unit count
- [ ] `add_course`:
  - Create course entry
  - Generate unique course code if needed
  - Set default values
  
- [ ] `add_course_units`:
  - Add/update units for a course
  - Support variable number of units (increase/decrease)
  - Include syllabus content
  - Link COs to units

**Example**:
```rust
pub async fn add_course(...) -> HttpResponse {
    let new_course = courses::ActiveModel {
        id: Set(Uuid::new_v4()),
        course_code: Set(request.course_code),
        course_name: Set(request.course_name),
        total_units: Set(request.total_units),
        is_active: Set(true),
        created_by: Set(Some(admin_id)),
        ...
    };
    
    let course = new_course.insert(db).await?;
    
    // Also create default Program Outcomes mapping
    ...
}
```

#### 3.2 Paper Format Management
**File**: `src/handlers/admin.rs`

**Tasks**:
- [ ] `upload_paper_format`:
  - Store format template with JSONB structure
  - Include header/footer templates
  - Define section requirements
  
- [ ] `get_paper_format`: Retrieve format by course
- [ ] `update_paper_format`: Modify existing format

**Format Structure**:
```json
{
  "header_template": "<h1>{UNIVERSITY_NAME}</h1>...",
  "footer_template": "Page {page_num}",
  "section_structure": {
    "sections": [
      {
        "name": "Section A - MCQ",
        "type": "MCQ",
        "count": 20,
        "marks_each": 1,
        "instructions": "Choose the correct answer"
      },
      {
        "name": "Section B - Short Answer",
        "type": "4_MARK",
        "count": 5,
        "marks_each": 4,
        "choice": "Answer any 4 out of 5"
      }
    ]
  }
}
```

#### 3.3 Question Search (Admin)
**File**: `src/handlers/admin.rs`

**Tasks**:
- [ ] Search across all users (not just own questions)
- [ ] Filter by course code and course name
- [ ] Show question creator information
- [ ] Export functionality

### Phase 4: Automatic Assignment Logic

#### 4.1 Bloom Level Assignment
**File**: `src/services/question.rs`

**Tasks**:
- [ ] Implement keyword-based analysis:
  ```rust
  // Keywords for each level
  let remember_words = ["define", "list", "name", "identify", "recall"];
  let understand_words = ["explain", "describe", "summarize", "interpret"];
  let apply_words = ["apply", "demonstrate", "use", "solve", "implement"];
  let analyze_words = ["analyze", "compare", "contrast", "differentiate"];
  let evaluate_words = ["evaluate", "assess", "justify", "critique"];
  let create_words = ["create", "design", "develop", "formulate"];
  ```

- [ ] Consider question marks (4-mark usually Apply/Understand, 12-mark usually Analyze/Evaluate)
- [ ] Check unit syllabus for context

#### 4.2 CO Assignment
**File**: `src/services/question.rs`

**Tasks**:
- [ ] Query `course_outcomes` table
- [ ] Match by:
  - Unit content
  - Bloom level
  - Question keywords
- [ ] Default to CO1 if ambiguous

**SQL Query**:
```sql
SELECT co.id 
FROM course_outcomes co
JOIN course_units cu ON cu.course_id = co.course_id
WHERE cu.id = $1  -- unit_id
AND co.bloom_level = $2  -- determined bloom level
LIMIT 1
```

#### 4.3 PO Mapping
**File**: Automatic via CO-PO mapping table

**Process**:
1. Once CO is assigned, POs are automatically linked via `co_po_mapping` table
2. No additional logic needed - database relationships handle this

### Phase 5: Paper Generation

#### 5.1 Question Selection Algorithm
**File**: `src/services/paper_gen.rs`

**Tasks**:
- [ ] Implement smart selection:
  ```rust
  async fn select_questions(
      course_id, 
      question_type, 
      count, 
      bloom_distribution
  ) -> Vec<Uuid> {
      // 1. Get all eligible questions
      // 2. Distribute across units evenly
      // 3. Match bloom level requirements
      // 4. Prefer less-used questions
      // 5. Ensure difficulty balance
      // 6. Shuffle and select
  }
  ```

**Selection Criteria**:
- Unit distribution: Equal representation from all units
- Bloom distribution: Match desired percentages
- Usage frequency: Prefer questions used less recently
- Difficulty: 30% Easy, 50% Medium, 20% Hard

#### 5.2 PDF Generation
**File**: `src/utils/pdf.rs`

**Tasks**:
- [ ] Set up PDF document with proper page size
- [ ] Add university header from template
- [ ] Add course information
- [ ] Create sections based on format
- [ ] Add questions with proper numbering
- [ ] Add Course Articulation Matrix table
- [ ] Add footer with page numbers
- [ ] Save to configured output directory

**Libraries to Use**:
- `printpdf` for PDF creation
- Font embedding for Unicode support
- Table layout for CAM

#### 5.3 CAM Generation
**File**: `src/services/cam.rs`

**Tasks**:
- [ ] Complete `generate_cam` function:
  ```rust
  pub async fn generate_cam(...) -> Result<Vec<CamEntry>> {
      // 1. Group questions by CO
      for co in course_outcomes {
          // 2. Find all questions mapped to this CO
          // 3. Calculate total marks
          // 4. Count bloom levels
          // 5. Get PO mappings from co_po_mapping
          // 6. Create CamEntry
      }
  }
  ```

- [ ] Format as HTML table for PDF
- [ ] Include summary statistics

### Phase 6: Testing & Deployment

#### 6.1 Unit Tests
**Files**: Create `tests/` directory

**Tasks**:
- [ ] Test authentication (password hashing, JWT)
- [ ] Test question CRUD operations
- [ ] Test duplicate detection
- [ ] Test paper generation logic
- [ ] Test CAM generation

#### 6.2 Integration Tests
**Tasks**:
- [ ] End-to-end user flow tests
- [ ] Admin workflow tests
- [ ] Database migration tests

#### 6.3 Documentation
**Tasks**:
- [ ] API documentation (consider using `utoipa` for OpenAPI)
- [ ] User manual
- [ ] Admin guide
- [ ] Deployment guide

## 📋 Implementation Checklist

### Database Setup
- [ ] Create PostgreSQL database
- [ ] Run `schema.sql`
- [ ] Verify all tables created
- [ ] Insert default admin user with proper password hash
- [ ] Insert Program Outcomes (PO1-PO12)

### Environment Configuration
- [ ] Copy `.env.example` to `.env`
- [ ] Configure database connection
- [ ] Set JWT secret (generate secure random string)
- [ ] Configure SMTP for emails
- [ ] Set PDF output directory

### Core Implementation
- [ ] Complete all entity models
- [ ] Implement remaining question handlers
- [ ] Implement admin handlers
- [ ] Complete automatic assignment logic
- [ ] Implement paper generation
- [ ] Implement CAM generation

### Testing
- [ ] Write unit tests for all services
- [ ] Write integration tests
- [ ] Manual testing with Postman/curl
- [ ] Load testing for concurrent users

### Deployment
- [ ] Build release binary: `cargo build --release`
- [ ] Set up systemd service
- [ ] Configure reverse proxy (nginx/Apache)
- [ ] Set up SSL certificates
- [ ] Configure backups

## 🔧 Development Workflow

### 1. Set Up Database
```bash
# Create database
createdb exam_automation

# Run schema
psql -d exam_automation -f schema.sql

# Create admin user (in psql)
INSERT INTO users (id, email, password_hash, salt, role, is_first_login)
VALUES (
    uuid_generate_v4(),
    'admin@example.com',
    -- Generate hash properly using Rust code
    '$argon2id$v=19$m=19456,t=2,p=1$...',
    'base64_salt_here',
    'admin',
    false
);
```

### 2. Build and Run
```bash
# Development
cargo run

# Production
cargo build --release
./target/release/exam_paper_automation
```

### 3. Test API
```bash
# Login
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@example.com","password":"Admin@123"}'

# Add question (with JWT token)
curl -X POST http://localhost:8080/api/user/questions/4-mark \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "course_code": "CS101",
    "unit_number": 1,
    "question_text": "Explain polymorphism with an example",
    "difficulty_level": "Medium"
  }'
```

## 🎯 Priority Order

1. **Highest Priority**: Complete entity models (needed for everything)
2. **High Priority**: Complete question management (core feature)
3. **Medium Priority**: Admin course management
4. **Medium Priority**: Automatic assignment logic
5. **Low Priority**: Paper generation and PDF creation

## 💡 Tips & Best Practices

1. **Database Transactions**: Use transactions for multi-step operations
2. **Error Handling**: Always return proper HTTP status codes
3. **Logging**: Use `tracing` for debugging
4. **Validation**: Validate all user inputs
5. **Security**: Never expose internal errors to users
6. **Testing**: Test each feature as you build it
7. **Documentation**: Comment complex logic

## 🆘 Troubleshooting

### Common Issues

1. **Database Connection Failed**
   - Check DATABASE_URL in .env
   - Verify PostgreSQL is running
   - Check network connectivity

2. **JWT Token Invalid**
   - Verify JWT_SECRET matches between requests
   - Check token expiration
   - Ensure Authorization header format: "Bearer TOKEN"

3. **Email Not Sending**
   - Verify SMTP configuration
   - Check firewall rules
   - For Gmail: Use App Password, not regular password

4. **Compilation Errors**
   - Run `cargo clean`
   - Update dependencies: `cargo update`
   - Check Rust version: `rustc --version`

## 📞 Next Steps

1. Start with completing the entity models
2. Test each feature thoroughly as you implement
3. Use the provided schema and API structure
4. Follow the implementation patterns in existing code
5. Refer to the README for architecture details

Good luck with the implementation! 🚀
