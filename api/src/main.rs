use {
    api::main_config,
    std::{io::{Result, Write}},
    chrono::Local,
    actix_web::{
        App, HttpServer, middleware,
    },
};


#[actix_web::main]
async fn main() -> Result<()> {
    configure_logger();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::new("%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
            .configure(main_config)
    }).bind("127.0.0.1:8080")?.run().await
}

/// Set configurations for the app logger
fn configure_logger() {
    std::env::set_var("RUST_LOG", "actix_web=info");

    env_logger::builder()
        .format(|formatter, record| {
            writeln!(formatter, "{} [{}] - {}", Local::now().format("%Y-%m-%dT%H:%M:%S"),
                     record.level(), record.args())
        })
        .init();
}
