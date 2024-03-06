use rest_example::{configuration::get_configuration, database::Database, startup::run, telemetry::{get_subscriber, init_subscriber}};
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod", "info", std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration");
    let db_client = Database::init(&configuration.database)
        .await
        .expect("Failed to connect to SurrealDB");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, db_client)?.await
}
