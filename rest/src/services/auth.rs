use actix_identity::Identity;
use actix_web::{
    delete, get, post,
    web::{Data, Json, ServiceConfig},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use minibar::auth::Credentials;
use minibar_rest::{AuthResponse, State};

pub fn auth(cfg: &mut ServiceConfig) {
    cfg.service(get_auth).service(login).service(logout);
}

#[get("/auth")]
pub async fn get_auth(user: Identity) -> impl Responder {
    Json(AuthResponse {
        user: user.id().unwrap(),
    })
}

#[post("/auth")]
pub async fn login(
    req: HttpRequest,
    credentials: Json<Credentials>,
    state: Data<State>,
) -> impl Responder {
    let name = credentials.name.to_owned();
    let mut session_lock = state.session_lock.lock().unwrap();

    if session_lock.contains(&name) {
        HttpResponse::Conflict().into()
    } else {
        session_lock.insert(name.to_owned());
        Identity::login(&req.extensions(), name.to_owned()).unwrap();

        HttpResponse::Created().json(AuthResponse {
            user: credentials.name.to_owned(),
        })
    }
}

#[delete("/auth")]
pub async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok()
}
