use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdatePizzaURL {
    pub uuid: Uuid,
}
