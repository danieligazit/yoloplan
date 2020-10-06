use {
    actix_web::{
        post, get, HttpResponse, Responder,
        web::{
            ServiceConfig, Json
        }
    }
};
use crate::routes::trip::models::FindTripConfig;

mod models;

/// Endpoint for finding the trip
#[post("/trip/")]
async fn find_trips(request_body: Json<FindTripConfig>) -> impl Responder {
    HttpResponse::Ok().body(format!("{} -> {}", request_body.from_iso_datetime,
                                    request_body.to_iso_datetime))
}

#[get("/trip/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to trip index")
}

pub fn config(service_config: &mut ServiceConfig) {
    service_config
        .service(find_trips)
        .service(index);
}