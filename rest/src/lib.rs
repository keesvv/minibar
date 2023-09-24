use minibar::Beverage;

pub mod routes;

#[derive(Clone)]
pub struct State {
    pub beverages: Vec<Beverage>,
}
