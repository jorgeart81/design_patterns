use crate::abstract_factory::burger::Hamburger;

pub struct BeefHamburger;

impl Hamburger for BeefHamburger {
    fn prepare(&self) {
        println!("Preparando una hamburguesa de carne.");
    }
}
