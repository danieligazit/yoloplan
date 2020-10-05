use {
    std::{io::Result},
    actix_web::{
        get, App, HttpResponse, HttpServer
    }
};
use actix_web::Responder;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Yoloplan API")
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new().service(index)
    }).bind("127.0.0.1:8080")?.run().await
}
