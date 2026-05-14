use crate::abstract_factory::burger::Hamburger;

pub struct ChickenHamburger;

impl Hamburger for ChickenHamburger {
    fn prepare(&self) {
        println!("Preparando una hamburguesa de pollo.");
    }
}
