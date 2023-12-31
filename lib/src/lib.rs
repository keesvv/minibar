pub mod auth;
pub mod instance;
pub mod order;
pub mod webhook;

use serde::{Deserialize, Serialize};
use uom::si::f32::Volume;

pub type BeverageId = String;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Beverage {
    pub id: BeverageId,
    pub description: String,
    pub capacity: Volume,
    pub amount: f32,
    #[serde(default)]
    pub metadata: BeverageMetadata,
    #[serde(default)]
    pub capabilities: Vec<BeverageCapability>,
}

impl Beverage {
    pub fn get_remaining(&self) -> Volume {
        self.amount * self.capacity
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum BeveragePackaging {
    Bottle,
    Can,
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
#[serde(rename_all = "camelCase", default)]
pub struct BeverageMetadata {
    pub category: Option<BeverageCategory>,
    pub image_uri: Option<String>,
    pub alc_percent: Option<f32>,
    pub packaging: Option<BeveragePackaging>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum BeverageCapability {
    Unit,
    Shot,
    Mix,
}
