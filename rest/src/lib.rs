use std::collections::HashSet;
use std::sync::Mutex;

use minibar::{auth::User, instance::Config, Beverage};

use serde::Serialize;

pub struct State {
    pub config: Config,
    pub beverages: Vec<Beverage>,
    pub webhook_url: Option<String>,
    pub session_lock: Mutex<HashSet<String>>,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub user: User,
}
