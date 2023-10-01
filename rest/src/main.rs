use std::env;
use std::{io, net::Ipv4Addr};

use actix_identity::IdentityMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use minibar::Beverage;
use minibar_rest::{Config, State};

use actix_cors::Cors;
use actix_web::{
    web::{delete, get, post, Data},
    App, HttpServer,
};

mod routes;

#[actix_web::main]
async fn main() {
    let webhook_url = env::var("WEBHOOK_URL").ok();
    let beverages: Vec<Beverage> = serde_json::from_reader(io::stdin()).unwrap();
    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(State {
                config: Config::default(),
                beverages: beverages.clone(),
                webhook_url: webhook_url.clone(),
            }))
            .wrap(Cors::permissive())
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .route("/auth", post().to(routes::login))
            .route("/auth", delete().to(routes::logout))
            .route("/auth", get().to(routes::get_auth))
            .route("/config", get().to(routes::get_config))
            .route("/beverages", get().to(routes::get_beverages))
            .route("/orders", post().to(routes::order))
    })
    .bind((Ipv4Addr::LOCALHOST, 8080))
    .unwrap()
    .run()
    .await
    .unwrap();
}
