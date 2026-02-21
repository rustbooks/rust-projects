use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    #[sea_orm(unique)]
    pub email: String,
    
    pub password_hash: String,
    pub salt: String,
    
    #[sea_orm(column_type = "String(Some(20))")]
    pub role: UserRole,
    
    pub is_first_login: bool,
    
    #[sea_orm(nullable)]
    pub reset_token: Option<String>,
    
    #[sea_orm(nullable)]
    pub reset_token_expiry: Option<DateTimeWithTimeZone>,
    
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(20))")]
pub enum UserRole {
    #[sea_orm(string_value = "admin")]
    Admin,
    #[sea_orm(string_value = "user")]
    User,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::questions::Entity")]
    Questions,
    
    #[sea_orm(has_many = "super::courses::Entity")]
    Courses,
}

impl Related<super::questions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Questions.def()
    }
}

impl Related<super::courses::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Courses.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
