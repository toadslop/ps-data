use std::path::PathBuf;

use actix_web::{App, HttpServer};
use config::env::Env;
use routes::health;

mod config;
mod routes;

fn init_logger(config_path: PathBuf) {
    log4rs::init_file(&config_path, Default::default()).expect(&format!(
        "the log config file was not found at {}",
        config_path.display()
    ));

    log::info!("Log config loaded at {}", config_path.display());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = Env::init();
    init_logger(env.get_config_path());

    HttpServer::new(|| App::new().service(health))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
