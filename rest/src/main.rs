mod routes;
mod services;

use std::env;
use std::{io, net::Ipv4Addr};

use actix_identity::IdentityMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use minibar::Beverage;
use minibar_rest::State;

use actix_cors::Cors;
use actix_web::{
    web::{get, post, Data},
    App, HttpServer,
};

#[actix_web::main]
async fn main() {
    let webhook_url = env::var("WEBHOOK_URL").ok();
    let beverages: Vec<Beverage> = serde_json::from_reader(io::stdin()).unwrap();
    let secret_key = Key::generate();

    let state = Data::new(State {
        config: Default::default(),
        beverages: beverages.clone(),
        webhook_url: webhook_url.clone(),
        session_lock: Default::default(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(Cors::permissive())
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .configure(services::auth)
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
