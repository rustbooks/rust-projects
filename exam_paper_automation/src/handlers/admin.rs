use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// GET /api/admin/questions/search
pub async fn search_questions(
    req: HttpRequest,
    data: web::Data<std::sync::Arc<crate::AppState>>,
) -> HttpResponse {
    HttpResponse::NotImplemented().json(ErrorResponse {
        error: "Admin search - all questions across all users".to_string(),
    })
}

/// POST /api/admin/paper-format
pub async fn upload_paper_format(
    req: HttpRequest,
    data: web::Data<std::sync::Arc<crate::AppState>>,
) -> HttpResponse {
    HttpResponse::NotImplemented().json(ErrorResponse {
        error: "Upload paper format template".to_string(),
    })
}

/// GET /api/admin/paper-format/{course_id}
pub async fn get_paper_format(
    req: HttpRequest,
    data: web::Data<std::sync::Arc<crate::AppState>>,
) -> HttpResponse {
    HttpResponse::NotImplemented().json(ErrorResponse {
        error: "Get paper format by course".to_string(),
    })
}

/// PUT /api/admin/paper-format/{id}
pub async fn update_paper_format(
    req: HttpRequest,
    data: web::Data<std::sync::Arc<crate::AppState>>,
) -> HttpResponse {
    HttpResponse::NotImplemented().json(ErrorResponse {
        error: "Update paper format".to_string(),
    })
}

/// GET /api/admin/courses
pub async fn list_courses(
    req: HttpRequest,
    data: web::Data<std::sync::Arc<crate::AppState>>,
) -> HttpResponse {
    HttpResponse::NotImplemented().json(ErrorResponse {
        error: "List all courses".to_string(),
    })
}

/// POST /api/admin/courses
pub async fn add_course(
    req: HttpRequest,
    data: web::Data<std::sync::Arc<crate::AppState>>,
) -> HttpResponse {
    HttpResponse::NotImplemented().json(ErrorResponse {
        error: "Add new course with code and name".to_string(),
    })
}

/// POST /api/admin/courses/{id}/units
pub async fn add_course_units(
    req: HttpRequest,
    data: web::Data<std::sync::Arc<crate::AppState>>,
) -> HttpResponse {
    HttpResponse::NotImplemented().json(ErrorResponse {
        error: "Add syllabus units to course".to_string(),
    })
}

/// POST /api/admin/generate-paper/{course_id}
pub async fn generate_paper(
    req: HttpRequest,
    data: web::Data<std::sync::Arc<crate::AppState>>,
) -> HttpResponse {
    HttpResponse::NotImplemented().json(ErrorResponse {
        error: "Generate exam paper PDF with CAM table".to_string(),
    })
}
