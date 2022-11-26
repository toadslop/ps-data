use actix_web::{get, HttpResponse, Responder};

pub mod admin;
pub mod general;

#[get("/")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json("Server is running.")
}
