use actix_web::{web::Data, web::Json, HttpResponse, Responder};
use minibar::order::Order;

use crate::State;

pub async fn get_beverages(state: Data<State>) -> impl Responder {
    Json(state.beverages.clone())
}

pub async fn get_config(state: Data<State>) -> impl Responder {
    Json(state.config.clone())
}

pub async fn order(order: Json<Order>) -> impl Responder {
    dbg!(order);
    HttpResponse::Created()
}
