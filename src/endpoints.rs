use std::error::Error;
use actix_web::{get, post, delete, put, HttpResponse, Responder, web};
use crate::database;
use crate::schema::FlightPlan;
use web::{Path, Json};


#[get("/api/v1/flightplan")]
pub async fn get_all_flight_plan() -> impl Responder {
    match database::get_all_flight_plans().unwrap() {
        Some(flight_plan_list) => {
            return HttpResponse::Ok()
                .content_type("application/json")
                .json(flight_plan_list);
        }
        None => {
            HttpResponse::NoContent()
                .body("There are no flight plans in the system");
        }
    }
}


#[get("/api/v1/flightplan/{flight_plan_id}")]
pub async fn get_flight_plan_by_id(id: Path<String>) -> impl Responder {
    let flight_plan_id = id.into_inner();
    let db_result = database::get_flight_plan_by_id(flight_plan_id.clone()).unwrap();
    match db_result {
        None => HttpResponse::NotFound().body(format!("There is no flight plan with id {flight_plan_id}")),
        Some(flight_plan) => return HttpResponse::Ok().json(flight_plan),
    }
}


#[delete("/api/v1/flightplan/{flight_plan_id}")]
pub async fn delete_flight_plan_by_id(id: Path<String>) -> impl Responder {
    let flight_plan_id = id.into_inner();
    match database::delete_flight_plan(flight_plan_id.clone()) {
        Ok(successful) => {
            if successful {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound()
                    .body(format!("There is no flight plan with id {flight_plan_id}"))
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}


#[post("/api/v1/flightplan")]
pub async fn file_flight_plan(flight_plan: Json<FlightPlan>) -> impl Responder {
    match database::insert_flight_plan(flight_plan.into_inner()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}


#[put("/api/v1/flightplan")]
pub async fn update_flight_plan(flight_plan: Json<FlightPlan>) -> impl Responder {
    let flight_plan_id = flight_plan.into_inner();
    match database::update_flight_plan(flight_plan_id.clone()) {
        Ok(succeeded) => {
            if succeeded {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound()
                    .body(format!("There is no flight plan with id {flight_plan_id}"))
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}