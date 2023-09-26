use serde::{Deserialize, Serialize};

pub type Order = Vec<OrderItem>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum OrderItem {
    Water,
}
