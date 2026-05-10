use crate::factory_method::init::initialize;

mod beef_hamburger;
mod burger;
mod chicken_hamburger;
mod init;

pub fn run_factory_method() {
    let restaurant = initialize();
    restaurant.order_hamburger();
}
