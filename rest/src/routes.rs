use actix_web::{web::Json, Responder};
use minibar::{Beverage, BeverageCategory, BeverageMetadata};
use uom::si::volume::{liter, Volume};

pub async fn get_beverages() -> impl Responder {
    Json(vec![
        Beverage {
            id: "whiskey".into(),
            description: "Whiskey".into(),
            capacity: Volume::new::<liter>(0.75),
            amount: 1. / 3.,
            metadata: BeverageMetadata {
                category: Some(BeverageCategory::Spirit),
                alc_percent: Some(0.4),
                ..Default::default()
            },
        },
        Beverage {
            id: "beer".into(),
            description: "Pilsner".into(),
            capacity: Volume::new::<liter>(0.3),
            amount: 6.0,
            metadata: BeverageMetadata {
                category: Some(BeverageCategory::Beer),
                alc_percent: Some(0.05),
                ..Default::default()
            },
        },
    ])
}
