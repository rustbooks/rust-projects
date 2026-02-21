-- Exam Paper Automation System - PostgreSQL 17 Schema
-- Author: Claude
-- Date: 2026-02-14

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ============================================
-- USERS & AUTHENTICATION
-- ============================================

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    salt TEXT NOT NULL,
    role VARCHAR(20) NOT NULL CHECK (role IN ('admin', 'user')),
    is_first_login BOOLEAN DEFAULT TRUE,
    reset_token TEXT,
    reset_token_expiry TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role ON users(role);

-- ============================================
-- COURSE MANAGEMENT
-- ============================================

CREATE TABLE courses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    course_code VARCHAR(50) UNIQUE NOT NULL,
    course_name TEXT NOT NULL,
    total_units INTEGER NOT NULL DEFAULT 5,
    is_active BOOLEAN DEFAULT TRUE,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_courses_code ON courses(course_code);
CREATE INDEX idx_courses_active ON courses(is_active);

-- ============================================
-- SYLLABUS & LEARNING OUTCOMES
-- ============================================

CREATE TABLE course_units (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    unit_number INTEGER NOT NULL,
    unit_name TEXT NOT NULL,
    unit_content TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(course_id, unit_number)
);

CREATE INDEX idx_course_units_course ON course_units(course_id);

-- Course Outcomes (CO)
CREATE TABLE course_outcomes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    co_number INTEGER NOT NULL,
    co_description TEXT NOT NULL,
    bloom_level VARCHAR(20) NOT NULL CHECK (bloom_level IN ('Remember', 'Understand', 'Apply', 'Analyze', 'Evaluate', 'Create')),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(course_id, co_number)
);

CREATE INDEX idx_co_course ON course_outcomes(course_id);

-- Program Outcomes (PO)
CREATE TABLE program_outcomes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    po_number INTEGER UNIQUE NOT NULL,
    po_description TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- CO-PO Mapping
CREATE TABLE co_po_mapping (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    co_id UUID NOT NULL REFERENCES course_outcomes(id) ON DELETE CASCADE,
    po_id UUID NOT NULL REFERENCES program_outcomes(id) ON DELETE CASCADE,
    correlation_level INTEGER CHECK (correlation_level IN (1, 2, 3)), -- 1=Low, 2=Medium, 3=High
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(co_id, po_id)
);

CREATE INDEX idx_co_po_co ON co_po_mapping(co_id);
CREATE INDEX idx_co_po_po ON co_po_mapping(po_id);

-- ============================================
-- QUESTION BANK
-- ============================================

CREATE TABLE questions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    unit_id UUID NOT NULL REFERENCES course_units(id) ON DELETE CASCADE,
    question_type VARCHAR(20) NOT NULL CHECK (question_type IN ('MCQ', '4_MARK', '12_MARK')),
    question_text TEXT NOT NULL,
    marks INTEGER NOT NULL,
    bloom_level VARCHAR(20) NOT NULL CHECK (bloom_level IN ('Remember', 'Understand', 'Apply', 'Analyze', 'Evaluate', 'Create')),
    co_id UUID REFERENCES course_outcomes(id) ON DELETE SET NULL,
    difficulty_level VARCHAR(20) CHECK (difficulty_level IN ('Easy', 'Medium', 'Hard')),
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_questions_course ON questions(course_id);
CREATE INDEX idx_questions_unit ON questions(unit_id);
CREATE INDEX idx_questions_type ON questions(question_type);
CREATE INDEX idx_questions_created_by ON questions(created_by);
CREATE INDEX idx_questions_text_search ON questions USING gin(to_tsvector('english', question_text));

-- MCQ Options (only for MCQ type questions)
CREATE TABLE mcq_options (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    question_id UUID NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    option_text TEXT NOT NULL,
    is_correct BOOLEAN DEFAULT FALSE,
    option_order INTEGER NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(question_id, option_order)
);

CREATE INDEX idx_mcq_question ON mcq_options(question_id);

-- Question-PO Mapping (since one question can map to multiple POs)
CREATE TABLE question_po_mapping (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    question_id UUID NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    po_id UUID NOT NULL REFERENCES program_outcomes(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(question_id, po_id)
);

CREATE INDEX idx_qpo_question ON question_po_mapping(question_id);
CREATE INDEX idx_qpo_po ON question_po_mapping(po_id);

-- ============================================
-- PAPER FORMAT & TEMPLATES
-- ============================================

CREATE TABLE paper_formats (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    format_name VARCHAR(255) NOT NULL,
    header_template TEXT, -- HTML/Template for header
    footer_template TEXT, -- HTML/Template for footer
    section_structure JSONB, -- JSON structure for paper sections
    total_marks INTEGER NOT NULL DEFAULT 100,
    mcq_count INTEGER DEFAULT 0,
    four_mark_count INTEGER DEFAULT 0,
    twelve_mark_count INTEGER DEFAULT 0,
    instructions TEXT,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(course_id, format_name)
);

CREATE INDEX idx_paper_formats_course ON paper_formats(course_id);

-- ============================================
-- GENERATED PAPERS
-- ============================================

CREATE TABLE generated_papers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    paper_format_id UUID REFERENCES paper_formats(id) ON DELETE SET NULL,
    paper_title VARCHAR(255) NOT NULL,
    exam_date DATE,
    paper_content JSONB, -- Stores question IDs and structure
    pdf_path TEXT,
    generated_by UUID REFERENCES users(id) ON DELETE SET NULL,
    generated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_generated_papers_course ON generated_papers(course_id);
CREATE INDEX idx_generated_papers_date ON generated_papers(exam_date);

-- ============================================
-- AUDIT & HISTORY
-- ============================================

CREATE TABLE question_history (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    question_id UUID NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    action VARCHAR(20) NOT NULL CHECK (action IN ('CREATED', 'UPDATED', 'DELETED')),
    old_data JSONB,
    new_data JSONB,
    changed_by UUID REFERENCES users(id) ON DELETE SET NULL,
    changed_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_question_history_question ON question_history(question_id);
CREATE INDEX idx_question_history_date ON question_history(changed_at);

-- ============================================
-- COURSE ARTICULATION MATRIX (CAM)
-- ============================================

CREATE TABLE course_articulation_matrix (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    question_id UUID NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    co_id UUID NOT NULL REFERENCES course_outcomes(id) ON DELETE CASCADE,
    bloom_level VARCHAR(20) NOT NULL,
    marks_contribution DECIMAL(5,2) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(course_id, question_id)
);

CREATE INDEX idx_cam_course ON course_articulation_matrix(course_id);
CREATE INDEX idx_cam_question ON course_articulation_matrix(question_id);

-- ============================================
-- TRIGGERS FOR AUTOMATIC UPDATES
-- ============================================

-- Trigger to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_courses_updated_at BEFORE UPDATE ON courses 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_course_units_updated_at BEFORE UPDATE ON course_units 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_questions_updated_at BEFORE UPDATE ON questions 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_paper_formats_updated_at BEFORE UPDATE ON paper_formats 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Trigger to log question changes
CREATE OR REPLACE FUNCTION log_question_changes()
RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'UPDATE') THEN
        INSERT INTO question_history(question_id, action, old_data, new_data, changed_by)
        VALUES (NEW.id, 'UPDATED', row_to_json(OLD), row_to_json(NEW), NEW.created_by);
        RETURN NEW;
    ELSIF (TG_OP = 'DELETE') THEN
        INSERT INTO question_history(question_id, action, old_data, changed_by)
        VALUES (OLD.id, 'DELETED', row_to_json(OLD), OLD.created_by);
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ language 'plpgsql';

CREATE TRIGGER log_question_updates AFTER UPDATE OR DELETE ON questions 
    FOR EACH ROW EXECUTE FUNCTION log_question_changes();

-- ============================================
-- VIEWS FOR REPORTING
-- ============================================

-- View for user question history
CREATE OR REPLACE VIEW v_user_question_history AS
SELECT 
    q.id,
    q.question_text,
    q.question_type,
    q.marks,
    q.bloom_level,
    c.course_code,
    c.course_name,
    cu.unit_number,
    cu.unit_name,
    co.co_number,
    u.email as created_by_email,
    q.created_at,
    q.updated_at
FROM questions q
JOIN courses c ON q.course_id = c.id
JOIN course_units cu ON q.unit_id = cu.id
LEFT JOIN course_outcomes co ON q.co_id = co.id
JOIN users u ON q.created_by = u.id
ORDER BY q.created_at DESC;

-- View for course articulation matrix report
CREATE OR REPLACE VIEW v_course_articulation_report AS
SELECT 
    c.course_code,
    c.course_name,
    co.co_number,
    co.co_description,
    q.question_type,
    q.bloom_level,
    COUNT(q.id) as question_count,
    SUM(q.marks) as total_marks
FROM courses c
JOIN course_outcomes co ON c.id = co.course_id
JOIN questions q ON c.id = q.course_id AND co.id = q.co_id
GROUP BY c.course_code, c.course_name, co.co_number, co.co_description, q.question_type, q.bloom_level
ORDER BY c.course_code, co.co_number;

-- ============================================
-- INITIAL SEED DATA
-- ============================================

-- Insert default Program Outcomes
INSERT INTO program_outcomes (po_number, po_description) VALUES
(1, 'Engineering knowledge: Apply knowledge of mathematics, science, engineering fundamentals'),
(2, 'Problem analysis: Identify, formulate, review literature, and analyze complex engineering problems'),
(3, 'Design/development of solutions: Design solutions for complex engineering problems'),
(4, 'Conduct investigations of complex problems: Use research-based knowledge and methods'),
(5, 'Modern tool usage: Create, select, and apply appropriate techniques and modern engineering tools'),
(6, 'The engineer and society: Apply reasoning informed by contextual knowledge'),
(7, 'Environment and sustainability: Understand impact of solutions in societal and environmental contexts'),
(8, 'Ethics: Apply ethical principles and commit to professional ethics'),
(9, 'Individual and team work: Function effectively as an individual and team member'),
(10, 'Communication: Communicate effectively on complex engineering activities'),
(11, 'Project management and finance: Demonstrate knowledge and understanding of management principles'),
(12, 'Life-long learning: Recognize the need for, and have the preparation for independent learning');

-- Create default admin user (password: Admin@123)
-- Salt and hash should be generated properly in application
INSERT INTO users (email, password_hash, salt, role, is_first_login) VALUES
('admin@example.com', 'changeme', 'changeme', 'admin', FALSE);

COMMENT ON TABLE users IS 'User authentication and authorization';
COMMENT ON TABLE courses IS 'Master course information';
COMMENT ON TABLE course_units IS 'Syllabus units for each course';
COMMENT ON TABLE course_outcomes IS 'Course-specific learning outcomes with Bloom taxonomy';
COMMENT ON TABLE program_outcomes IS 'Institution-level program outcomes (POs)';
COMMENT ON TABLE questions IS 'Question bank with automatic bloom/CO/PO mapping';
COMMENT ON TABLE paper_formats IS 'Paper format templates for automated generation';
COMMENT ON TABLE generated_papers IS 'History of generated exam papers';
COMMENT ON TABLE question_history IS 'Audit trail for question modifications';
COMMENT ON TABLE course_articulation_matrix IS 'CO-PO mapping matrix for accreditation';
