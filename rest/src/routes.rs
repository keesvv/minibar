use actix_identity::Identity;
use actix_web::{web::Data, web::Json, HttpMessage, HttpRequest, HttpResponse, Responder};
use minibar::{order::Order, webhook::WebhookBody};
use minibar_rest::State;
use serde::Deserialize;
use serde_json::json;

pub async fn get_beverages(_: Identity, state: Data<State>) -> impl Responder {
    Json(state.beverages.clone())
}

pub async fn get_config(_: Identity, state: Data<State>) -> impl Responder {
    Json(state.config.clone())
}

pub async fn get_auth(user: Identity) -> impl Responder {
    Json(json!({ "user": user.id().unwrap() }))
}

#[derive(Deserialize)]
pub struct Credentials {
    pub name: String,
}

pub async fn login(req: HttpRequest, credentials: Json<Credentials>) -> impl Responder {
    Identity::login(&req.extensions(), credentials.name.to_owned()).unwrap();
    HttpResponse::Created()
}

pub async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok()
}

pub async fn order(state: Data<State>, user: Identity, order: Json<Order>) -> impl Responder {
    if let Some(url) = &state.webhook_url {
        reqwest::Client::new()
            .post(url)
            .header("X-Minibar-Webhook", "abc" /* signature goes here */)
            .json(&WebhookBody {
                order: order.0,
                who: user.id().unwrap(),
            })
            .send()
            .await
            .unwrap();
    }
    HttpResponse::Created()
}
