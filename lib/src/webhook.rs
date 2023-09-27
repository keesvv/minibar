use serde::{Deserialize, Serialize};

use crate::{auth::User, order::Order};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebhookBody {
    pub who: User,
    pub order: Order,
}
