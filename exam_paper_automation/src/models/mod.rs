// Entity models generated from PostgreSQL schema
pub mod users;
pub mod courses;
pub mod course_units;
pub mod course_outcomes;
pub mod program_outcomes;
pub mod questions;
pub mod mcq_options;
pub mod paper_formats;
pub mod generated_papers;
pub mod question_history;

// Re-export commonly used types
pub use users::Entity as Users;
pub use courses::Entity as Courses;
pub use course_units::Entity as CourseUnits;
pub use course_outcomes::Entity as CourseOutcomes;
pub use program_outcomes::Entity as ProgramOutcomes;
pub use questions::Entity as Questions;
pub use mcq_options::Entity as McqOptions;
pub use paper_formats::Entity as PaperFormats;
pub use generated_papers::Entity as GeneratedPapers;
pub use question_history::Entity as QuestionHistory;
