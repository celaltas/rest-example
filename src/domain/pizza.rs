
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct Pizza {
    pub uuid: String,
    pub name: String,
}


impl Pizza {
    pub fn new(uuid: String, name: String) -> Pizza {
        Pizza { uuid, name }
    }
}
