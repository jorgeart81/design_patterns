use crate::factory_method::init::initialize;

mod factory_method;

fn main() {
    let restaurant = initialize();
    restaurant.order_hamburger();
}
