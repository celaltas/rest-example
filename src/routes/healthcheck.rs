use actix_web::{get, HttpResponse, Responder};


#[get("/v1/healthcheck")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().finish()
}