use std::io;

use crate::{
    abstract_factory::run_abstract_factory_with_static_dispatch, adapter::run_adapter,
    builder::run_builder, factory_method::run_factory_method, prototype::run_prototype,
};

mod abstract_factory;
mod adapter;
mod builder;
mod factory_method;
mod prototype;

fn main() {
    loop {
        println!("\n=== MENÚ DE PATRONES DE DISEÑO ===");
        println!("1. Adapter");
        println!("2. Abstract Factory");
        println!("3. Builder");
        println!("4. Factory Method");
        println!("5. Prototype");
        println!("6. Salir");
        println!("==================================");
        println!("Selecciona una opción: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error leyendo la línea");

        let choice = input.trim().to_lowercase();

        match choice.as_str() {
            "1" | "adapter" => run_adapter(),
            "2" | "abstract_factory" => run_abstract_factory_with_static_dispatch(),
            "3" | "builder" => run_builder(),
            "4" | "factory_method" => run_factory_method(),
            "5" | "prototype" => run_prototype(),
            "6" | "exit" | "salir" => {
                println!("¡Hasta luego!");
                break;
            }
            _ => {
                println!("Opción no válida. Intenta de nuevo.\n");
            }
        }
    }
}
