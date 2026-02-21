use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "questions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub course_id: Uuid,
    pub unit_id: Uuid,
    
    #[sea_orm(column_type = "String(Some(20))")]
    pub question_type: QuestionType,
    
    #[sea_orm(column_type = "Text")]
    pub question_text: String,
    
    pub marks: i32,
    
    #[sea_orm(column_type = "String(Some(20))")]
    pub bloom_level: BloomLevel,
    
    #[sea_orm(nullable)]
    pub co_id: Option<Uuid>,
    
    #[sea_orm(column_type = "String(Some(20))", nullable)]
    pub difficulty_level: Option<DifficultyLevel>,
    
    pub created_by: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(20))")]
pub enum QuestionType {
    #[sea_orm(string_value = "MCQ")]
    MCQ,
    #[sea_orm(string_value = "4_MARK")]
    FourMark,
    #[sea_orm(string_value = "12_MARK")]
    TwelveMark,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(20))")]
pub enum BloomLevel {
    #[sea_orm(string_value = "Remember")]
    Remember,
    #[sea_orm(string_value = "Understand")]
    Understand,
    #[sea_orm(string_value = "Apply")]
    Apply,
    #[sea_orm(string_value = "Analyze")]
    Analyze,
    #[sea_orm(string_value = "Evaluate")]
    Evaluate,
    #[sea_orm(string_value = "Create")]
    Create,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(20))")]
pub enum DifficultyLevel {
    #[sea_orm(string_value = "Easy")]
    Easy,
    #[sea_orm(string_value = "Medium")]
    Medium,
    #[sea_orm(string_value = "Hard")]
    Hard,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::CreatedBy",
        to = "super::users::Column::Id"
    )]
    User,
    
    #[sea_orm(
        belongs_to = "super::courses::Entity",
        from = "Column::CourseId",
        to = "super::courses::Column::Id"
    )]
    Course,
    
    #[sea_orm(
        belongs_to = "super::course_units::Entity",
        from = "Column::UnitId",
        to = "super::course_units::Column::Id"
    )]
    CourseUnit,
    
    #[sea_orm(has_many = "super::mcq_options::Entity")]
    McqOptions,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::courses::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Course.def()
    }
}

impl Related<super::course_units::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CourseUnit.def()
    }
}

impl Related<super::mcq_options::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::McqOptions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
