use {
    actix_web::{
        web, Responder, HttpResponse, get
    }
};


#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to yoloplan API :)")
}


pub fn config(service_config: &mut web::ServiceConfig) {
    service_config.service(index);
}