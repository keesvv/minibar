use actix_web::{web::Data, web::Json, Responder};

use crate::State;

pub async fn get_beverages(state: Data<State>) -> impl Responder {
    Json(state.beverages.clone())
}
