use actix_web::{App, HttpServer};

mod schema;
mod database;
mod endpoints;

use env_logger::Env;
use actix_web::middleware::Logger;

use crate::endpoints::{
    get_all_flight_plans, get_flight_plan_by_id,
    delete_flight_plan_by_id, file_flight_plan,
    update_flight_plan
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info")); 
    HttpServer::new(move || {
        App::new()
            .service(get_flight_plan_by_id)
            .service(get_all_flight_plans)
            .service(delete_flight_plan_by_id)
            .service(file_flight_plan)
            .service(update_flight_plan)
            .wrap(Logger::default())
    })
        .bind(("127.0.0.1", 3000))?
        .workers(2)
        .run()
        .await
}