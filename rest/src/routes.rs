use actix_web::{web::Data, web::Json, web::ReqData, HttpResponse, Responder};
use minibar::{auth::User, order::Order, webhook::WebhookBody};
use minibar_rest::State;
use serde_json::json;

pub async fn get_beverages(state: Data<State>) -> impl Responder {
    Json(state.beverages.clone())
}

pub async fn get_config(state: Data<State>) -> impl Responder {
    Json(state.config.clone())
}

pub async fn get_auth(user: ReqData<User>) -> impl Responder {
    Json(json!({ "user": *user }))
}

pub async fn order(state: Data<State>, user: ReqData<User>, order: Json<Order>) -> impl Responder {
    if let Some(url) = &state.webhook_url {
        reqwest::Client::new()
            .post(url)
            .header("X-Minibar-Webhook", "abc" /* signature goes here */)
            .json(&WebhookBody {
                order: order.0,
                who: user.to_string(),
            })
            .send()
            .await
            .unwrap();
    }
    HttpResponse::Created()
}
