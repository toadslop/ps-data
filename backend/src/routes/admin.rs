use actix_web::{
    get, guard,
    middleware::{NormalizePath, TrailingSlash},
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

use super::RoleGuard;

pub fn admin_config(cfg: &mut ServiceConfig) {
    log::trace!("Initializing admin scope.");

    cfg.service(
        web::scope("/admin")
            .guard(guard::Host("localhost")) // TODO: load host from env variable
            .guard(RoleGuard::with_role("admin"))
            .service(admin_root)
            .wrap(NormalizePath::new(TrailingSlash::Always)),
    );
}

#[get("/")]
async fn admin_root() -> impl Responder {
    log::info!("Root admin path hit");

    HttpResponse::Ok().json("Hit admin root.")
}
