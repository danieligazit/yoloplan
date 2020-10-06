use {
    api::main_config,
    std::{io::{Result, Write}},
    chrono::Local,
    actix_web::{
        App, HttpServer, middleware,
    },
};


const SERVER_ADDRESS: &str = "127.0.0.1:8080";
const LOG_FORMAT: &str = "%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T";



/// API entry point
#[actix_web::main]
async fn main() -> Result<()> {
    config_logger();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::new(LOG_FORMAT))
            .configure(main_config)
    }).bind(SERVER_ADDRESS)?.run().await
}

fn config_logger() {
    std::env::set_var("RUST_LOG", "actix_web=info");

    env_logger::builder()
        .format(|formatter, record| {
            writeln!(formatter, "{} [{}] - {}", Local::now().format("%Y-%m-%dT%H:%M:%S"),
                     record.level(), record.args())
        })
        .init();
}
