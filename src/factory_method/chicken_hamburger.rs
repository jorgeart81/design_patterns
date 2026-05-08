use crate::factory_method::burger::{Hamburger, Restaurant};

pub struct ChickenHamburger;

impl Hamburger for ChickenHamburger {
    fn prepare(&self) {
        println!("Preparando una hamburguesa de pollo.");
    }
}

pub struct ChickenRestaurant;

impl Restaurant for ChickenRestaurant {
    fn create_hambuger(&self) -> Box<dyn Hamburger> {
        Box::new(ChickenHamburger)
    }
}
