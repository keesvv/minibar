use minibar::{Beverage, BeverageCategory};
use uom::si::volume::{liter, Volume};

fn main() {
    let b = Beverage {
        name: "Whiskey".to_string(),
        category: BeverageCategory::Spirit,
        capacity: Volume::new::<liter>(0.75),
        amount: 1. / 3.,
    };

    dbg!(&b);
    dbg!(&b.get_remaining());
}
