use actix_identity::Identity;
use actix_web::{web::Data, web::Json, HttpResponse, Responder};
use minibar::{
    order::Order,
    webhook::{WebhookPayload, HEADER_SIGNATURE},
};
use minibar_rest::State;

pub async fn get_config(state: Data<State>) -> impl Responder {
    Json(state.config.clone())
}

pub async fn get_beverages(_: Identity, state: Data<State>) -> impl Responder {
    Json(state.beverages.clone())
}

pub async fn order(state: Data<State>, user: Identity, order: Json<Order>) -> impl Responder {
    if let Some(url) = &state.webhook_url {
        reqwest::Client::new()
            .post(url)
            .header(HEADER_SIGNATURE, "abc" /* signature goes here */)
            .json(&WebhookPayload {
                order: order.into_inner(),
                who: user.id().unwrap(),
            })
            .send()
            .await
            .unwrap();
    }
    HttpResponse::Created()
}
