use serde::{Deserialize, Serialize};

use crate::BeverageId;

pub type Order = Vec<OrderItem>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum OrderItem {
    Water,
    #[serde(rename_all = "camelCase")]
    Unit {
        beverage_id: BeverageId,
    },
    #[serde(rename_all = "camelCase")]
    Shot {
        beverage_id: BeverageId,
    },
    #[serde(rename_all = "camelCase")]
    Mix {
        beverage_ids: Vec<BeverageId>,
    },
}
