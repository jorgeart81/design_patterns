use crate::factory_method::burger::{Hamburger, Restaurant};

pub struct BeefHamburger;

impl Hamburger for BeefHamburger {
    fn prepare(&self) {
        println!("Preparando una hamburguesa de carne.");
    }
}

pub struct BeefRestaurant;

impl Restaurant for BeefRestaurant {
    fn create_hambuger(&self) -> Box<dyn Hamburger> {
        Box::new(BeefHamburger)
    }
}
