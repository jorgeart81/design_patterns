use crate::abstract_factory::drink::Drink;

pub struct Soda;

impl Drink for Soda {
    fn pour(&self) {
        println!("Sirviendo un vaso con gaseosa.");
    }
}
