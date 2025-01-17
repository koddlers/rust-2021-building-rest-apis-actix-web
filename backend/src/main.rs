#![allow(unused)]

use actix_cors::Cors;
use actix_web::{App, Error, HttpMessage, HttpServer};
use actix_web::dev::ServiceRequest;
use actix_web::middleware::Logger;
use env_logger::Env;
use config::Config;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use actix_web_httpauth::extractors::bearer::{BearerAuth, self};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::endpoints::{
    delete_flight_plan_by_id, file_flight_plan, get_all_flight_plan, get_flight_plan_by_id,
    new_user, update_flight_plan,
};
use crate::schema::User;

mod schema;
mod database;
mod endpoints;

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = req.app_data::<bearer::Config>()
        .cloned()
        .unwrap_or_default()
        .scope("");

    match database::get_user(String::from(credentials.token())) {
        Ok(user) => {
            match user {
                None => {
                    Err((AuthenticationError::from(config).into(), req))
                }
                Some(_) => {
                    req.extensions_mut().insert(user);
                    Ok(req)
                }
            }
        }
        Err(_) => {
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

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
        let authentication_validator = HttpAuthentication::bearer(validator);
        App::new()
            .service(get_flight_plan_by_id)
            .service(get_all_flight_plan)
            .service(delete_flight_plan_by_id)
            .service(file_flight_plan)
            .service(update_flight_plan)
            .service(new_user)
            .wrap(authentication_validator)
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_methods(vec!["OPTIONS", "GET", "POST", "PUT", "DELETE"])
                    .allow_any_origin()
                    .allowed_origin("localhost:63342")
                    .allowed_origin("localhost:3000")
                    .allowed_origin("localhost:3001")
                    .max_age(3600)
            )
    })
        .bind(("0.0.0.0", 3000))?
        .bind_openssl("0.0.0.0:3001", ssl_builder)?
        .workers(2)
        .run()
        .await
}
