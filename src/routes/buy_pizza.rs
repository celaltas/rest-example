use crate::{
    database::{Database,PizzaDataTrait},
    domain::{buy_pizza_request::BuyPizzaRequest, pizza::Pizza},
    error::PizzaError,
};
use actix_web::{
    post,
    web::{Data, Json},
};
use uuid::Uuid;
use validator::Validate;

#[post("/v1/pizza")]
pub async fn buy_pizza(
    body: Json<BuyPizzaRequest>,
    db: Data<Database>,
) -> Result<Json<Pizza>, PizzaError> {
    match body.validate() {
        Ok(_) => {
            let uuid = Uuid::new_v4().to_string();
            let name = body.name.clone();
            let new_pizza = Pizza::new(uuid, name);
            let result = db.create_pizza(&new_pizza).await;
            match result {
                Some(created) => Ok(Json(created)),
                None => Err(PizzaError::PizzaCreationFailure),
            }
        }
        Err(_) => Err(PizzaError::PizzaCreationFailure),
    }
}
