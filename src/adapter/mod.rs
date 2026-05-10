mod adaptee;
mod target;

pub use adaptee::SpecificTarget;
pub use target::OrdinaryTarget;
pub use target::Target;

/// Converts adaptee's specific interface to a compatible `Target` output.
pub struct TargetAdapter {
    adaptee: SpecificTarget,
}

impl TargetAdapter {
    pub fn new(adaptee: SpecificTarget) -> Self {
        Self { adaptee }
    }
}

impl Target for TargetAdapter {
    fn request(&self) -> String {
        // Here's the "adaptation" of a specific output to a compatible output.
        self.adaptee.specific_request().chars().rev().collect()
    }
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

pub fn run_adapter() {
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
