use std::{io, net::Ipv4Addr};

use minibar::Beverage;
use minibar_rest::{routes, Config, State};

use actix_cors::Cors;
use actix_web::{
    web::{get, Data},
    App, HttpServer,
};

#[actix_web::main]
async fn main() {
    let beverages: Vec<Beverage> = serde_json::from_reader(io::stdin()).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(State {
                config: Config::default(),
                beverages: beverages.clone(),
            }))
            .wrap(Cors::permissive())
            .route("/beverages", get().to(routes::get_beverages))
            .route("/config", get().to(routes::get_config))
    })
    .bind((Ipv4Addr::LOCALHOST, 8080))
    .unwrap()
    .run()
    .await
    .unwrap();
}
