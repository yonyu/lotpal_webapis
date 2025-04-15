mod adapters;
mod usecases;
mod config;
mod domain;
mod api_docs;

use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::{
    adapters::db::PgCn649Repository,
    usecases::service::Cn649Service,
    api_docs::ApiDoc
};
use crate::adapters::controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = config::get_database_url();
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let repository = PgCn649Repository::new(pool);
    let service = Cn649Service::new(repository);
    let service_data = web::Data::new(service);

    HttpServer::new(move || {
        App::new()
            .app_data(service_data.clone())
            // Swagger UI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            // API routes
            .route("/cn649", web::get().to(controllers::get_all))
            .route("/cn649/{id}", web::get().to(controllers::get_by_id))
            .route("/cn649", web::post().to(controllers::create))
            .route("/cn649/{id}", web::put().to(controllers::update))
            .route("/cn649/{id}", web::delete().to(controllers::delete))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}