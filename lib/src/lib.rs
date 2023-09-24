use serde::{Deserialize, Serialize};
use uom::si::f32::Volume;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Beverage {
    pub id: String,
    pub description: String,
    pub capacity: Volume,
    pub amount: f32,
    #[serde(default)]
    pub metadata: BeverageMetadata,
    #[serde(default)]
    pub capabilities: BeverageCapabilities,
}

impl Beverage {
    pub fn get_remaining(&self) -> Volume {
        self.amount * self.capacity
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum BeverageCategory {
    Softdrink,
    Beer,
    Wine,
    Spirit,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BeverageMetadata {
    pub category: Option<BeverageCategory>,
    pub image_uri: Option<String>,
    pub alc_percent: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct BeverageCapabilities {
    pub can_shot: bool,
}
