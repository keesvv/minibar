use std::fmt::{self, Display};

use minibar::order::{Order, OrderItem};

pub struct OrderFormatter<'a> {
    order: &'a Order,
}

impl<'a> OrderFormatter<'a> {
    pub fn new(order: &'a Order) -> Self {
        Self { order }
    }

    fn fmt_item(item: &OrderItem) -> String {
        match item {
            OrderItem::Water => "water".into(),
            OrderItem::Unit { beverage_id } => beverage_id.to_owned(),
            OrderItem::Shot { beverage_id } => format!("{} shot", beverage_id),
            OrderItem::Mix { beverage_ids } => format!("{} mix", beverage_ids.join(" + ")),
        }
    }
}

impl<'a> Display for OrderFormatter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.order
                .iter()
                .map(Self::fmt_item)
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
