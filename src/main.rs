#![allow(unused)]

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;

use crate::endpoints::{delete_flight_plan_by_id, file_flight_plan, get_all_flight_plan, get_flight_plan_by_id, new_user, update_flight_plan};

mod schema;
mod database;
mod endpoints;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .service(get_flight_plan_by_id)
            .service(get_all_flight_plan)
            .service(delete_flight_plan_by_id)
            .service(file_flight_plan)
            .service(update_flight_plan)
            .service(new_user)
            .wrap(Logger::default())
    })
        .bind(("0.0.0.0", 3000))?
        .workers(2)
        .run()
        .await
}
