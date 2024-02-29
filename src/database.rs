use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

use crate::configuration::DatabaseSettings;



#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init(setting: &DatabaseSettings) -> Result<Database, Error> {
        let address = format!("{}:{}", setting.host, setting.port);
        let client = Surreal::new::<Ws>(address).await?;
        client
            .signin(Root {
                username: &setting.username,
                password: &setting.password,
            })
            .await?;

        Ok(Database {
            client: client,
            name_space: setting.namespace.clone(),
            db_name: setting.database_name.clone(),
        })
    }
}
