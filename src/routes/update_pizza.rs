use crate::domain::update_pizza_url::UpdatePizzaURL;
use actix_web::{patch, web::Path, HttpResponse, Responder};

#[patch("/v1/pizza{uuid}")]
pub async fn update_pizza(path: Path<UpdatePizzaURL>) -> impl Responder {
    let uuid = path.into_inner().uuid;

    HttpResponse::Ok().finish()
}
