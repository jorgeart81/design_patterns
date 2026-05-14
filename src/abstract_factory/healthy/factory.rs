use crate::abstract_factory::{
    burger::Hamburger,
    drink::Drink,
    healthy::{burger::ChickenHamburger, drink::Water},
    restaurant_factory::{RestaurantFactory, RestaurantFactoryDyn},
};

pub struct HealthyRestaurant;

impl RestaurantFactory for HealthyRestaurant {
    type D = Water;

    type H = ChickenHamburger;

    fn create_drink(&self) -> Self::D {
        Water
    }

    fn create_hamburger(&self) -> Self::H {
        ChickenHamburger
    }
}

impl RestaurantFactoryDyn for HealthyRestaurant {
    fn create_hamburger(&self) -> Box<dyn Hamburger> {
        Box::new(ChickenHamburger)
    }

    fn create_drink(&self) -> Box<dyn Drink> {
        Box::new(Water)
    }
}
