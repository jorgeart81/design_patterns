pub trait Hamburger {
    fn prepare(&self);
}

pub trait Restaurant {
    fn create_hambuger(&self) -> Box<dyn Hamburger>;

    fn order_hamburger(&self) {
        let hamburger = self.create_hambuger();
        hamburger.prepare();
    }
}
