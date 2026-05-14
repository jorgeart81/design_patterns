use crate::abstract_factory::drink::Drink;

pub struct Water;

impl Drink for Water {
    fn pour(&self) {
        println!("Sirviendo un vaso con agua.");
    }
}
