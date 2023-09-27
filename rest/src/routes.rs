use actix_web::{web::Data, web::Json, HttpResponse, Responder};
use minibar::{order::Order, webhook::WebhookBody};
use minibar_rest::State;

pub async fn get_beverages(state: Data<State>) -> impl Responder {
    Json(state.beverages.clone())
}

pub async fn get_config(state: Data<State>) -> impl Responder {
    Json(state.config.clone())
}

pub async fn order(state: Data<State>, order: Json<Order>) -> impl Responder {
    if let Some(url) = &state.webhook_url {
        reqwest::Client::new()
            .post(url)
            .header("X-Minibar-Webhook", "abc" /* signature goes here */)
            .json(&WebhookBody {
                order: order.0,
                who: "User".into(), /* use provided authentication */
            })
            .send()
            .await
            .unwrap();
    }
    HttpResponse::Created()
}
