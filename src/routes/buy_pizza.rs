use crate::domain::buy_pizza_request::BuyPizzaRequest;
use actix_web::{post, web::Json, HttpResponse, Responder};
use validator::Validate;

#[post("/v1/pizza")]
pub async fn buy_pizza(body: Json<BuyPizzaRequest>) -> impl Responder {
    match body.validate() {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
