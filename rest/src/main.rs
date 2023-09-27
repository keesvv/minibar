use std::env;
use std::{io, net::Ipv4Addr};

use minibar::Beverage;
use minibar_rest::{Config, State};

use actix_cors::Cors;
use actix_web::{
    web::{get, post, Data},
    App, HttpServer,
};

mod routes;

#[actix_web::main]
async fn main() {
    let webhook_url = env::var("WEBHOOK_URL").ok();
    let beverages: Vec<Beverage> = serde_json::from_reader(io::stdin()).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(State {
                config: Config::default(),
                beverages: beverages.clone(),
                webhook_url: webhook_url.clone(),
            }))
            .wrap(Cors::permissive())
            .route("/beverages", get().to(routes::get_beverages))
            .route("/config", get().to(routes::get_config))
            .route("/orders", post().to(routes::order))
    })
    .bind((Ipv4Addr::LOCALHOST, 8080))
    .unwrap()
    .run()
    .await
    .unwrap();
}
