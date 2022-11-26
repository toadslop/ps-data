use actix_web::{
    get,
    middleware::{NormalizePath, TrailingSlash},
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

use super::RoleGuard;

pub fn user_config(cfg: &mut ServiceConfig) {
    log::trace!("Initializing user scope");
    cfg.service(
        web::scope("/user")
            .guard(RoleGuard::with_role("user"))
            .service(user_root)
            .wrap(NormalizePath::new(TrailingSlash::Always)),
    );
}

#[get("/")]
async fn user_root() -> impl Responder {
    log::info!("User scope root endpoint hit");

    HttpResponse::Ok().json("Hit user root.")
}
