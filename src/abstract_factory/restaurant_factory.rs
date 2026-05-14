use crate::abstract_factory::{burger::Hamburger, drink::Drink};

/// Static dispatch using associated types.
/// - Uses associated types (`type D`, `type H`) for concrete types
/// - Resolved at compile-time → no vtable overhead
/// - Higher performance but less flexible (type fixed at implementation)
pub trait RestaurantFactory {
    type D: Drink;
    type H: Hamburger;

    fn create_drink(&self) -> Self::D;
    fn create_hamburger(&self) -> Self::H;
}

/// Dynamic dispatch using trait objects.
/// - Uses trait objects (`Box<dyn Hamburger>`) for dynamic types
/// - Resolved at runtime via vtable
/// - More flexible but with vtable lookup overhead
pub trait RestaurantFactoryDyn {
    fn create_hamburger(&self) -> Box<dyn Hamburger>;
    fn create_drink(&self) -> Box<dyn Drink>;
}
