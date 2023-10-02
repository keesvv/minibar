use serde::{Deserialize, Serialize};

use crate::{auth::User, order::Order};

pub const HEADER_SIGNATURE: &str = "X-Minibar-Webhook";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebhookPayload {
    pub who: User,
    pub order: Order,
}
