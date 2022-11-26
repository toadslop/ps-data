use std::path::PathBuf;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use config::env::Env;

mod config;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

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

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
