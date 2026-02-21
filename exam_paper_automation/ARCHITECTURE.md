# System Architecture - Exam Paper Automation

## 🏛️ High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Client Layer                             │
│  (Web Browser / Mobile App / API Client)                        │
└────────────────────────────┬────────────────────────────────────┘
                             │ HTTPS
                             │ JWT Token
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                      API Gateway Layer                           │
│                     (Actix-web 4.9)                             │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  Middleware Stack:                                        │  │
│  │  • CORS                                                   │  │
│  │  • Logger                                                 │  │
│  │  • Compression                                           │  │
│  │  • Authentication (JWT Validation)                       │  │
│  │  • Authorization (Role-Based)                            │  │
│  └──────────────────────────────────────────────────────────┘  │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Application Layer                            │
│                        (Rust)                                    │
│  ┌──────────────┐  ┌──────────────┐  ┌─────────────────────┐  │
│  │   Handlers   │  │   Services   │  │    Utilities        │  │
│  │              │  │              │  │                     │  │
│  │ • Auth       │  │ • Question   │  │ • Password Hash     │  │
│  │ • User       │  │ • Paper Gen  │  │ • JWT Token         │  │
│  │ • Admin      │  │ • CAM Gen    │  │ • Email Service     │  │
│  │              │  │              │  │ • PDF Generator     │  │
│  └──────┬───────┘  └──────┬───────┘  └─────────────────────┘  │
│         │                  │                                     │
│         └──────────┬───────┘                                     │
│                    │                                             │
│                    ▼                                             │
│         ┌──────────────────────┐                                │
│         │   SeaORM (ORM)       │                                │
│         │   • Type-safe        │                                │
│         │   • Async/Await      │                                │
│         │   • Connection Pool  │                                │
│         └──────────┬───────────┘                                │
└────────────────────┼────────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────────┐
│                   Data Layer                                     │
│                 PostgreSQL 17                                    │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  Core Tables:                                             │  │
│  │  • users (authentication, roles)                         │  │
│  │  • courses (course management)                           │  │
│  │  • course_units (syllabus units)                         │  │
│  │  • questions (question bank)                             │  │
│  │  • course_outcomes (COs with Bloom)                      │  │
│  │  • program_outcomes (PO1-PO12)                           │  │
│  │  • co_po_mapping (CO-PO relationships)                   │  │
│  │  • paper_formats (templates)                             │  │
│  │  • generated_papers (paper history)                      │  │
│  │  • question_history (audit trail)                        │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

## 🔄 Data Flow Diagrams

### 1. Authentication Flow
```
User → [Login Request] → API
                          ↓
                    Verify Password
                    (Argon2 + Salt)
                          ↓
                    Generate JWT Token
                          ↓
                    Return Token + Role
                          ↓
User ← [Token + User Info] ←
```

### 2. Add Question Flow
```
User → [Add Question] → Middleware (Verify JWT)
                              ↓
                        Extract User Claims
                              ↓
                        Validate Input
                              ↓
                        Check Duplicate
                              ↓
                        Auto-assign Bloom Level
                              ↓
                        Auto-assign CO
                              ↓
                        Save to Database
                              ↓
                        Log to History
                              ↓
User ← [Question Created] ←
```

### 3. Paper Generation Flow
```
Admin → [Generate Paper] → Verify Admin Role
                                ↓
                          Load Paper Format
                                ↓
                          Select Questions
                          • Unit Distribution
                          • Bloom Balance
                          • Difficulty Mix
                                ↓
                          Generate CAM
                                ↓
                          Create PDF
                          • Header/Footer
                          • Sections
                          • Questions
                          • CAM Table
                                ↓
                          Save Paper Record
                                ↓
Admin ← [PDF Download Link] ←
```

## 📊 Database Schema Relationships

```
users
  ├─→ questions (created_by)
  └─→ courses (created_by)

courses
  ├─→ course_units
  ├─→ course_outcomes
  ├─→ questions
  ├─→ paper_formats
  └─→ generated_papers

course_units
  └─→ questions

course_outcomes
  ├─→ questions (co_id)
  └─→ co_po_mapping

program_outcomes
  └─→ co_po_mapping

questions
  ├─→ mcq_options (for MCQ type)
  ├─→ question_history (audit)
  └─→ question_po_mapping

paper_formats
  └─→ generated_papers
```

## 🎯 Feature Modules

### User Module
```
┌─────────────────────────────┐
│      User Features          │
├─────────────────────────────┤
│ 1. Add 4-Mark Questions     │
│ 2. Add 12-Mark Questions    │
│ 3. Add MCQ Questions        │
│ 4. Search Questions         │
│ 5. View Question History    │
│ 6. Edit Own Questions       │
└─────────────────────────────┘
        │
        ├─→ Automatic Processing:
        │   • Duplicate Detection
        │   • Bloom Level Assignment
        │   • CO Mapping
        │   • PO Mapping (via CO)
        │   • History Logging
        └─→
```

### Admin Module
```
┌─────────────────────────────┐
│     Admin Features          │
├─────────────────────────────┤
│ 1. Manage Courses           │
│    • Add Course             │
│    • Add Units              │
│    • Define Syllabus        │
│                             │
│ 2. Search All Questions     │
│    • Filter by Course       │
│    • View by Creator        │
│                             │
│ 3. Paper Format Management  │
│    • Upload Templates       │
│    • Edit Formats           │
│                             │
│ 4. Generate Papers          │
│    • Auto-select Questions  │
│    • Generate CAM           │
│    • Export PDF             │
└─────────────────────────────┘
```

## 🔐 Security Architecture

```
┌─────────────────────────────────────────┐
│         Security Layers                 │
├─────────────────────────────────────────┤
│                                         │
│  1. Authentication                      │
│     • Argon2 Password Hashing           │
│     • Per-User Salt                     │
│     • JWT Tokens (24hr expiry)          │
│     • Email-based Password Reset        │
│                                         │
│  2. Authorization                       │
│     • Role-Based Access Control         │
│     • Middleware Guards                 │
│     • Resource Ownership Checks         │
│                                         │
│  3. Input Validation                    │
│     • Email Format                      │
│     • Password Strength                 │
│     • Question Text Length              │
│     • SQL Injection Prevention (ORM)    │
│                                         │
│  4. Data Protection                     │
│     • HTTPS Only (in production)        │
│     • CORS Configuration                │
│     • Token Expiration                  │
│     • Audit Trail (question_history)    │
│                                         │
└─────────────────────────────────────────┘
```

## 📈 Scalability Considerations

### Current Architecture (Single Server)
- Suitable for: 100-500 concurrent users
- Database: Single PostgreSQL instance
- File Storage: Local filesystem

### Future Scalability Options

1. **Horizontal Scaling**
   ```
   Load Balancer
        ├─→ App Server 1 ──┐
        ├─→ App Server 2 ──┼─→ PostgreSQL Primary
        └─→ App Server 3 ──┘       ├─→ Read Replica 1
                                    └─→ Read Replica 2
   ```

2. **Caching Layer**
   - Add Redis for:
     - Session storage
     - Frequently accessed courses
     - Question search results

3. **File Storage**
   - Move PDFs to S3/Cloud Storage
   - Use CDN for distribution

4. **Database Optimization**
   - Implement read replicas
   - Partition large tables
   - Use materialized views for CAM

## 🛠️ Technology Stack Summary

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Language** | Rust 1.93.0 | Core application logic |
| **Web Framework** | Actix-web 4.9 | HTTP server & routing |
| **ORM** | SeaORM 1.1 | Database abstraction |
| **Database** | PostgreSQL 17 | Data persistence |
| **Authentication** | Argon2 + JWT | Security |
| **Email** | Lettre | Password reset |
| **PDF** | printpdf | Document generation |
| **Async Runtime** | Tokio | Concurrency |
| **Serialization** | Serde | JSON handling |

## 📁 File Organization

```
exam_paper_automation/
│
├── schema.sql                    # Database schema
├── Cargo.toml                    # Dependencies
├── README.md                     # Full documentation
├── IMPLEMENTATION_GUIDE.md       # Implementation steps
├── QUICKSTART.md                 # Quick start guide
├── ARCHITECTURE.md               # This file
│
├── .env.example                  # Configuration template
│
├── examples/
│   └── generate_hash.rs          # Password hash utility
│
└── src/
    ├── main.rs                   # Application entry
    ├── config.rs                 # Configuration
    │
    ├── models/                   # Data models (SeaORM)
    │   ├── mod.rs
    │   ├── users.rs
    │   ├── courses.rs
    │   ├── questions.rs
    │   └── ...
    │
    ├── handlers/                 # HTTP handlers
    │   ├── mod.rs
    │   ├── auth.rs               # Login, password reset
    │   ├── user.rs               # User operations
    │   └── admin.rs              # Admin operations
    │
    ├── services/                 # Business logic
    │   ├── mod.rs
    │   ├── question.rs           # Question management
    │   ├── paper_gen.rs          # Paper generation
    │   └── cam.rs                # CAM generation
    │
    ├── middleware/               # Request middleware
    │   ├── mod.rs
    │   └── auth.rs               # Auth & admin guards
    │
    └── utils/                    # Utilities
        ├── mod.rs
        ├── auth.rs               # Password, JWT
        ├── email.rs              # Email service
        ├── validation.rs         # Input validation
        └── pdf.rs                # PDF generation
```

## 🎓 Key Design Decisions

### 1. Why Rust?
- **Type Safety**: Prevents many runtime errors
- **Performance**: Near C++ performance
- **Concurrency**: Fearless concurrency with Tokio
- **Memory Safety**: No garbage collection overhead

### 2. Why PostgreSQL 17?
- **JSONB Support**: Flexible schema for paper formats
- **Full-Text Search**: Built-in question search
- **Triggers**: Automatic audit logging
- **Views**: Simplified reporting queries

### 3. Why SeaORM?
- **Type-Safe**: Compile-time query validation
- **Async**: Perfect for Actix-web
- **Migration Support**: Version-controlled schema changes
- **Active Development**: Latest Rust ecosystem

### 4. Why Argon2?
- **Memory-Hard**: Resistant to GPU attacks
- **Configurable**: Tunable security parameters
- **Recommended**: Winner of Password Hashing Competition

## 🔮 Future Enhancements

1. **Machine Learning**
   - Auto-classify question difficulty
   - Suggest similar questions
   - Predict exam performance

2. **Collaboration**
   - Real-time collaborative editing
   - Question review workflow
   - Peer review system

3. **Analytics**
   - Question usage statistics
   - Bloom level distribution graphs
   - Student performance correlation

4. **Integration**
   - LMS integration (Moodle, Canvas)
   - Cloud storage (Google Drive, OneDrive)
   - Question import from Word/PDF

---

This architecture provides a solid foundation for a scalable, secure, and maintainable exam paper automation system. 🚀
