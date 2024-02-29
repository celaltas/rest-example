use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdatePizzaURL {
    pub uuid: String,
}
