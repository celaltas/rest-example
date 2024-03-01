use actix_web::{
    get,
    web::{Data, Json},
};

use crate::{database::Database, domain::pizza::Pizza, error::PizzaError};

#[get("/v1/pizza")]
pub async fn show_pizzas(db: Data<Database>) -> Result<Json<Vec<Pizza>>, PizzaError> {
    let pizzas = db.get_all_pizzas().await;

    match pizzas {
        Some(vals) => Ok(Json(vals)),
        None => Err(PizzaError::NoPizzasFound),
    }
}
