use actix_web::{
    get,
    guard::{self, Guard},
    HttpResponse, Responder,
};

pub mod admin;
pub mod user;

#[get("/")]
pub async fn health() -> impl Responder {
    log::info!("Health check endpoint hit");

    HttpResponse::Ok().json("Server is running.")
}

pub struct RoleGuard {
    _role: String,
}

impl RoleGuard {
    fn with_role<S: Into<String>>(role: S) -> RoleGuard {
        let role_string: String = role.into();
        log::debug!("Initializing role guard with role {}", &role_string);

        RoleGuard { _role: role_string }
    }
}

impl Guard for RoleGuard {
    fn check(&self, _ctx: &guard::GuardContext<'_>) -> bool {
        log::trace!("Checking role...");
        // TODO: complete this role guard.
        true
    }
}
