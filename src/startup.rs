use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

use crate::{
    database::Database,
    routes::{
        buy_pizza::buy_pizza,healthcheck::healthcheck,
        show_pizzas::show_pizzas, update_pizza::update_pizza,
    },
};

pub fn run(listener: TcpListener, db_client: Database) -> Result<Server, std::io::Error> {
    let db_client = web::Data::new(db_client);
    let server = HttpServer::new(move || {
        App::new()
            .service(healthcheck)
            .service(buy_pizza)
            .service(update_pizza)
            .service(show_pizzas)
            .app_data(db_client.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
