use actix_web::{delete, HttpResponse, Responder};


#[delete("/v1/pizza")]
pub async fn delete_pizza() -> impl Responder {
    HttpResponse::Ok().finish()
}
