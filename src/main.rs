use crate::{
    adapter::{OrdinaryTarget, SpecificTarget, Target, TargetAdapter},
    builder::{
        builders::{Builder, CarBuilder, CarManualBuilder},
        cars::{Car, Manual},
        director::Director,
    },
    factory_method::init::initialize,
};

mod adapter;
mod builder;
mod factory_method;

fn main() {
    run_adapter();
    run_builder();
    // run_factory_method();
}

fn run_factory_method() {
    let restaurant = initialize();
    restaurant.order_hamburger();
}

fn run_builder() {
    let mut car_builder = CarBuilder::default();

    // Director gets the concrete builder object from the client
    // (application code). That's because application knows better which
    // builder to use to get a specific product.
    Director::construct_sports_car(&mut car_builder);

    // The final product is often retrieved from a builder object, since
    // Director is not aware and not dependent on concrete builders and
    // products.
    let car: Car = car_builder.build();
    println!("Car built: {:?}\n", car.car_type());

    let mut manual_builder = CarManualBuilder::default();

    // Director may know several building recipes.
    Director::construct_sports_car(&mut manual_builder);

    // The final car manual.
    let manual: Manual = manual_builder.build();
    println!("Car manual built:\n{}", manual);
}

/// Calls any object of a `Target` trait.
///
/// To understand the Adapter pattern better, imagine that this is
/// a client code, which can operate over a specific interface only
/// (`Target` trait only). It means that an incompatible interface cannot be
/// passed here without an adapter.
fn call(target: impl Target) {
    println!("'{}'", target.request());
}

fn run_adapter() {
    let target = OrdinaryTarget;

    print!("A compatible target can be directly called: ");
    call(target);

    let adaptee = SpecificTarget;

    println!(
        "Adaptee is incompatible with client: '{}'",
        adaptee.specific_request()
    );

    let adapter = TargetAdapter::new(adaptee);

    print!("But with adapter client can call its method: ");
    call(adapter);
}
