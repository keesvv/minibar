use uom::si::f32::Volume;

#[derive(Debug)]
pub struct Beverage {
    pub name: String,
    pub category: BeverageCategory,
    pub capacity: Volume,
    pub amount: f32,
}

impl Beverage {
    pub fn get_remaining(&self) -> Volume {
        self.amount * self.capacity
    }
}

#[derive(Debug)]
pub enum BeverageCategory {
    Beer,
    Wine,
    Spirit,
}
