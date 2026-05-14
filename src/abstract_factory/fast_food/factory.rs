use crate::abstract_factory::{
    burger::Hamburger,
    drink::Drink,
    fast_food::{burger::BeefHamburger, drink::Soda},
    restaurant_factory::{RestaurantFactory, RestaurantFactoryDyn},
};

pub struct FastFoodRestaurant;

impl RestaurantFactory for FastFoodRestaurant {
    type D = Soda;

    type H = BeefHamburger;

    fn create_drink(&self) -> Self::D {
        Soda
    }

    fn create_hamburger(&self) -> Self::H {
        BeefHamburger
    }
}

impl RestaurantFactoryDyn for FastFoodRestaurant {
    fn create_hamburger(&self) -> Box<dyn Hamburger> {
        Box::new(BeefHamburger)
    }

    fn create_drink(&self) -> Box<dyn Drink> {
        Box::new(Soda)
    }
}
