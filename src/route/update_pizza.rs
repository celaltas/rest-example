use crate::{
    database::{Database,PizzaDataTrait},
    domain::{pizza::Pizza, update_pizza_url::UpdatePizzaURL},
    error::PizzaError,
};
use actix_web::{
    patch,
    web::{Data, Json, Path},
};

#[patch("/v1/pizza/{uuid}")]
pub async fn update_pizza(
    path: Path<UpdatePizzaURL>,
    db: Data<Database>,
) -> Result<Json<Pizza>, PizzaError> {
    let uuid = path.into_inner().uuid;
    match db.update_pizza(uuid).await {
        Some(updated) => Ok(Json(updated)),
        None => Err(PizzaError::NoSuchPizzaFound),
    }
}
