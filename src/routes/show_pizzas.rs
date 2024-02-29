use actix_web::{get, HttpResponse, Responder};


#[get("/v1/pizza")]
pub async fn show_pizzas() -> impl Responder {
    HttpResponse::Ok().finish()
}