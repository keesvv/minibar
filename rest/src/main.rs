use actix_cors::Cors;
use minibar_rest::routes;
use std::net::Ipv4Addr;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .route("/beverages", web::get().to(routes::get_beverages))
    })
    .bind((Ipv4Addr::LOCALHOST, 8080))
    .unwrap()
    .run()
    .await
    .unwrap();
}
