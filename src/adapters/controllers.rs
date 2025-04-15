use actix_web::{web, HttpResponse, Responder};
use crate::usecases::service::Cn649Service;
use crate::adapters::db::PgCn649Repository;

pub async fn get_all(
    service: web::Data<Cn649Service<PgCn649Repository>>
) -> impl Responder {
    match service.get_all().await {
        Ok(entries) => HttpResponse::Ok().json(entries),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_by_id(
    id: web::Path<i32>,
    service: web::Data<Cn649Service<PgCn649Repository>>
) -> impl Responder {
    match service.get_by_id(id.into_inner()).await {
        Ok(Some(entry)) => HttpResponse::Ok().json(entry),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create(
    entry: web::Json<crate::domain::models::Cn649>,
    service: web::Data<Cn649Service<PgCn649Repository>>
) -> impl Responder {
    match service.create(&entry.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update(
    id: web::Path<i32>,
    entry: web::Json<crate::domain::models::Cn649>, // entities::Cn649Entry
    service: web::Data<Cn649Service<PgCn649Repository>>
) -> impl Responder {
    match service.update(id.into_inner(), &entry.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete(
    id: web::Path<i32>,
    service: web::Data<Cn649Service<PgCn649Repository>>
) -> impl Responder {
    match service.delete(id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Similarly, implement CRUD endpoints for create, update, delete, and get_by_id.