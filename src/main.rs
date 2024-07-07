#![allow(unused)]

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;
use config::Config;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use crate::endpoints::{delete_flight_plan_by_id, file_flight_plan, get_all_flight_plan, get_flight_plan_by_id, new_user, update_flight_plan};

mod schema;
mod database;
mod endpoints;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .unwrap();

    let certificate_key = settings.get_string("CERTIFICATE_KEY").unwrap();
    let certificate = settings.get_string("CERTIFICATE").unwrap();

    let mut ssl_builder = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls())?;
    ssl_builder.set_private_key_file(certificate_key, SslFiletype::PEM).unwrap();
    ssl_builder.set_certificate_chain_file(certificate).unwrap();

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
            .wrap(
                Cors::default()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allow_any_origin()
                    .max_age(3600)
            )
    })
        .bind(("0.0.0.0", 3000))?
        .bind_openssl("0.0.0.0:3001", ssl_builder)?
        .workers(2)
        .run()
        .await
}
