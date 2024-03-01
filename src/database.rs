use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

use crate::configuration::DatabaseSettings;
use crate::domain::pizza::Pizza;

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

        client
            .use_ns(setting.namespace.clone())
            .use_db(setting.database_name.clone())
            .await
            .unwrap();

        Ok(Database {
            client: client,
            name_space: setting.namespace.clone(),
            db_name: setting.database_name.clone(),
        })
    }

    pub async fn get_all_pizzas(&self) -> Option<Vec<Pizza>> {
        match self.client.select("pizza").await {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }

    pub async fn create_pizza(&self, new_pizza: &Pizza) -> Option<Pizza> {
        let created_pizza = self
            .client
            .create(("pizza", new_pizza.uuid.clone().to_string()))
            .content(new_pizza)
            .await;
        match created_pizza {
            Ok(created) => created,
            Err(e) => {
                println!("error occured {:#?}", e.to_string());
                None
            }
        }
    }
}
