use std::{net::TcpListener, time::Duration};

use rest_example::{configuration::get_configuration, database::Database, startup::run};

pub struct TestApp {
    pub address: String,
    pub db_client: Database,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn buy_pizza(&self, body: serde_json::Value) -> reqwest::Response {
        self.http_client
            .post(format!("{}/v1/pizza", &self.address))
            .json(&body)
            .send()
            .await
            .expect("Failed to execute request")
    }
}

pub async fn spawn_app() -> TestApp {
    let configuration = get_configuration().expect("Failed to read configuration");
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let db_client = Database::init(&configuration.database)
        .await
        .expect("Failed to connect to SurrealDB");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener, db_client.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    let address = format!("http://127.0.0.1:{}", port);
    let timeout = Duration::from_millis(200);
    let http_client = reqwest::Client::builder().timeout(timeout).build().unwrap();

    TestApp {
        address,
        db_client,
        http_client,
    }
}
