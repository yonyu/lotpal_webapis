use actix_web::{web, HttpResponse, Responder};
use crate::usecases::service::Cn649Service;
use crate::adapters::db::PgCn649Repository;
use crate::domain::models::Cn649;

/// Get all CN649 entries
#[utoipa::path(
    get,
    path = "/cn649",
    responses(
        (status = 200, description = "List all CN649 entries", body = Vec<Cn649>),
        (status = 500, description = "Internal server error")
    ),
    tag = "cn649"
)]
pub async fn get_all(
    service: web::Data<Cn649Service<PgCn649Repository>>
) -> impl Responder {
    match service.get_all().await {
        Ok(entries) => HttpResponse::Ok().json(entries),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Get CN649 entry by ID
#[utoipa::path(
    get,
    path = "/cn649/{id}",
    responses(
        (status = 200, description = "Found CN649 entry", body = Cn649),
        (status = 404, description = "CN649 entry not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "CN649 entry ID")
    ),
    tag = "cn649"
)]
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

/// Create new CN649 entry
#[utoipa::path(
    post,
    path = "/cn649",
    request_body = Cn649,
    responses(
        (status = 201, description = "CN649 entry created successfully"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    tag = "cn649"
)]
pub async fn create(
    cn649: web::Json<Cn649>,
    service: web::Data<Cn649Service<PgCn649Repository>>
) -> impl Responder {
    match service.create(&cn649.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Update CN649 entry
#[utoipa::path(
    put,
    path = "/cn649/{id}",
    request_body = Cn649,
    responses(
        (status = 200, description = "CN649 entry updated successfully"),
        (status = 404, description = "CN649 entry not found"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "CN649 entry ID")
    ),
    tag = "cn649"
)]
pub async fn update(
    id: web::Path<i32>,
    cn649: web::Json<Cn649>,
    service: web::Data<Cn649Service<PgCn649Repository>>
) -> impl Responder {
    match service.update(id.into_inner(), &cn649.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Delete CN649 entry
#[utoipa::path(
    delete,
    path = "/cn649/{id}",
    responses(
        (status = 200, description = "CN649 entry deleted successfully"),
        (status = 404, description = "CN649 entry not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "CN649 entry ID")
    ),
    tag = "cn649"
)]
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