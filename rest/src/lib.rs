use minibar::Beverage;

use serde::Serialize;
use uom::si::f32::Volume;
use uom::si::volume::centiliter;

pub mod routes;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub size: SizeConfig,
    pub max_order_size: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            size: Default::default(),
            max_order_size: 3,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SizeConfig {
    pub shot: Volume,
}

impl Default for SizeConfig {
    fn default() -> Self {
        Self {
            shot: Volume::new::<centiliter>(3.0),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct State {
    pub config: Config,
    pub beverages: Vec<Beverage>,
}
