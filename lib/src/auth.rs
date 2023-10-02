use serde::Deserialize;

pub type User = String;

#[derive(Deserialize)]
pub struct Credentials {
    pub name: String,
}
