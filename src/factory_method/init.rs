use std::io;

use crate::factory_method::{
    beef_hamburger::BeefRestaurant, burger::Restaurant, chicken_hamburger::ChickenRestaurant,
};

pub fn initialize() -> &'static dyn Restaurant {
    let mut input = String::new();

    println!("¿Qué hamburguesa quieres? (carne/pollo)");
    io::stdin()
        .read_line(&mut input)
        .expect("Error leyendo la línea");

    let choice = input.trim().to_lowercase();

    match choice.as_str() {
        "carne" => &BeefRestaurant,
        "pollo" => &ChickenRestaurant,
        _ => panic!("Opción no válida"),
    }
}
