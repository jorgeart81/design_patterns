use std::io;

use crate::{adapter::run_adapter, builder::run_builder, factory_method::run_factory_method};

mod adapter;
mod builder;
mod factory_method;

fn main() {
    loop {
        println!("\n=== MENÚ DE PATRONES DE DISEÑO ===");
        println!("1. Adapter");
        println!("2. Builder");
        println!("3. Factory Method");
        println!("4. Salir");
        println!("==================================");
        println!("Selecciona una opción: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error leyendo la línea");

        let choice = input.trim().to_lowercase();

        match choice.as_str() {
            "1" | "adapter" => run_adapter(),
            "2" | "builder" => run_builder(),
            "3" | "factory_method" => run_factory_method(),
            "4" | "exit" | "salir" => {
                println!("¡Hasta luego!");
                break;
            }
            _ => {
                println!("Opción no válida. Intenta de nuevo.\n");
            }
        }
    }
}
