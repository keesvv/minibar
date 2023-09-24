use minibar::Beverage;

use serde::Serialize;
use uom::si::f32::Volume;
use uom::si::volume::centiliter;

pub mod routes;

#[derive(Debug, Clone, Serialize, Default)]
pub struct Config {
    pub size: SizeConfig,
}

#[derive(Debug, Clone, Serialize)]
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
