use actix_web::{web, HttpResponse, Responder};
use crate::usecases::service::Cn649Service;

pub async fn get_all(
    service: web::Data<Cn649Service<impl crate::usecases::repository::Cn649Repository>>,
) -> impl Responder {
    match service.get_all().await {
        Ok(entries) => HttpResponse::Ok().json(entries),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Similarly, implement CRUD endpoints for create, update, delete, and get_by_id.