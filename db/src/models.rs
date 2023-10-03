use diesel::prelude::*;
use minibar::BeverageId;
use uom::si::f32::Volume;

#[derive(Queryable)]
pub struct Beverage {
    pub id: BeverageId,
    pub description: String,
    pub capacity: Volume,
    pub amount: f32,
}
