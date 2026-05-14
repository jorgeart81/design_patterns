use std::io;

use crate::abstract_factory::{
    fast_food::factory::FastFoodRestaurant,
    healthy::factory::HealthyRestaurant,
    restaurant_factory::{RestaurantFactory, RestaurantFactoryDyn},
};

mod burger;
mod drink;
mod fast_food;
mod healthy;
mod restaurant_factory;

pub fn run_abstract_factory_with_static_dispatch() {
    let mut input = String::new();

    println!("¿Qué menú quieres? (1. saludable/ 2. comida rápida)");
    io::stdin()
        .read_line(&mut input)
        .expect("Error leyendo la línea");

    let choice = input.trim().to_lowercase();

    match choice.as_str() {
        "1" | "saludable" => order_with_static(HealthyRestaurant),
        "2" | "comida rápida" => order_with_static(FastFoodRestaurant),
        _ => panic!("Opción no válida"),
    }
}

fn order_with_static(factory: impl RestaurantFactory) {
    let hamburger = factory.create_hamburger(); // Concrete type: Self::H (e.g., ChickenHamburger)
    let drink = factory.create_drink(); // Concrete type: Self::D (e.g., Water)

    // IMPORTANT: In static dispatch, trait methods require importing the trait into scope.
    // Rust cannot resolve 'prepare'/'pour' because they are trait methods, not struct methods.
    // Example without use: error[E0599]: no method named `prepare` found
    use burger::Hamburger;
    use drink::Drink;

    hamburger.prepare();
    drink.pour();
}

pub fn run_abstract_factory_with_dinamyc_dispatch() {
    let mut input = String::new();

    println!("¿Qué menú quieres? (1. saludable/ 2. comida rápida)");
    io::stdin()
        .read_line(&mut input)
        .expect("Error leyendo la línea");

    let choice = input.trim().to_lowercase();

    match choice.as_str() {
        "1" | "saludable" => order_with_dynamic(&HealthyRestaurant),
        "2" | "comida rápida" => order_with_dynamic(&FastFoodRestaurant),
        _ => panic!("Opción no válida"),
    }
}

fn order_with_dynamic(factory: &dyn RestaurantFactoryDyn) {
    let hamburger = factory.create_hamburger(); // Type: Box<dyn Hamburger>
    let drink = factory.create_drink(); // Type: Box<dyn Drink>

    // NO 'use' needed: in dynamic dispatch, methods are resolved via vtable.
    // The trait is already associated with the dynamic type, Rust knows which methods to use.

    hamburger.prepare();
    drink.pour();
}
