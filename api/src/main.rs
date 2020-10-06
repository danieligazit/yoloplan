use {
    std::{io::Result},
    api::main_config,
    actix_web::{
        App, HttpServer, middleware
    }
};


#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(main_config)
    }).bind("127.0.0.1:8080")?.run().await
}
