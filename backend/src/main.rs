use actix_web::{App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use auth::validator;
use config::env::Env;
use routes::{admin::admin_config, health, user::user_config};
use std::path::PathBuf;

mod auth;
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
    let auth = HttpAuthentication::bearer(validator);
    init_logger(env.get_config_path()); // TODO: make sure logger provides thread information.

    HttpServer::new(|| {
        App::new()
            .service(health)
            .configure(admin_config)
            .configure(user_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
