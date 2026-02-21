use actix_web::{web, App, HttpServer, middleware};
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use tracing_subscriber;

mod config;
mod models;
mod handlers;
mod services;
mod middleware as app_middleware;
mod utils;

pub struct AppState {
    pub db: DatabaseConnection,
    pub config: config::Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Load configuration
    let cfg = config::Config::from_env().expect("Failed to load configuration");
    
    // Connect to database
    let db = Database::connect(&cfg.database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Connected to database successfully");

    // Create application state
    let state = Arc::new(AppState {
        db: db.clone(),
        config: cfg.clone(),
    });

    let server_host = cfg.server_host.clone();
    let server_port = cfg.server_port;

    tracing::info!("Starting server at {}:{}", server_host, server_port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            // Public routes
            .service(
                web::scope("/api/auth")
                    .route("/login", web::post().to(handlers::auth::login))
                    .route("/reset-password", web::post().to(handlers::auth::reset_password))
                    .route("/verify-token", web::post().to(handlers::auth::verify_reset_token))
                    .route("/change-password", web::post().to(handlers::auth::change_password))
            )
            // Protected User routes
            .service(
                web::scope("/api/user")
                    .wrap(app_middleware::auth::AuthMiddleware)
                    .route("/questions/4-mark", web::post().to(handlers::user::add_4_mark_question))
                    .route("/questions/12-mark", web::post().to(handlers::user::add_12_mark_question))
                    .route("/questions/mcq", web::post().to(handlers::user::add_mcq_question))
                    .route("/questions/search", web::get().to(handlers::user::search_questions))
                    .route("/questions/history", web::get().to(handlers::user::get_question_history))
                    .route("/questions/{id}", web::put().to(handlers::user::update_question))
            )
            // Protected Admin routes
            .service(
                web::scope("/api/admin")
                    .wrap(app_middleware::auth::AdminMiddleware)
                    .route("/questions/search", web::get().to(handlers::admin::search_questions))
                    .route("/paper-format", web::post().to(handlers::admin::upload_paper_format))
                    .route("/paper-format/{course_id}", web::get().to(handlers::admin::get_paper_format))
                    .route("/paper-format/{id}", web::put().to(handlers::admin::update_paper_format))
                    .route("/courses", web::get().to(handlers::admin::list_courses))
                    .route("/courses", web::post().to(handlers::admin::add_course))
                    .route("/courses/{id}/units", web::post().to(handlers::admin::add_course_units))
                    .route("/generate-paper/{course_id}", web::post().to(handlers::admin::generate_paper))
            )
    })
    .bind((server_host, server_port))?
    .run()
    .await
}
