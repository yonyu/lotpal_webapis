mod adapters;
mod usecases;
mod config;
mod domain;

use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use crate::{adapters::db::PgCn649Repository, usecases::service::Cn649Service};
use config::get_database_url;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = config::get_database_url();
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to database");

    let repository = PgCn649Repository::new(pool);
    let service = Cn649Service::new(repository);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(service.clone()))
            .route("/entries", web::get().to(crate::adapters::controllers::get_all))
            // Add routes for CRUD endpoints here
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
