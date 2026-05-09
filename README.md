# Rust Design Patterns

A practical demo project showcasing common design patterns implemented in Rust.

## Patterns Included

| Pattern | Description | Module |
|---------|-------------|--------|
| **Builder** | Separates object construction from representation, allowing different representations for the same construction process. | `src/builder/` |
| **Factory Method** | Defines an interface for creating objects, letting subclasses decide which class to instantiate. | `src/factory_method/` |

## Project Structure

```
design_patterns/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── builder/
    │   ├── mod.rs
    │   ├── components.rs      # Shared components
    │   ├── director.rs        # Director for construction steps
    │   ├── builders/
    │   │   ├── mod.rs         # Builder trait
    │   │   ├── car.rs         # CarBuilder
    │   │   └── car_manual.rs  # CarManualBuilder
    │   └── cars/
    │       ├── mod.rs
    │       ├── car.rs         # Car product
    │       └── manual.rs      # Manual product
    └── factory_method/
        ├── mod.rs
        ├── burger.rs          # Traits: Hamburger, Restaurant
        ├── beef_hamburger.rs  # BeefHamburger + BeefRestaurant
        ├── chicken_hamburger.rs
        └── init.rs            # Factory initialization
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed

## Execution

```bash
cargo build
cargo run
```

## Sample Output

```
Car built: SportsCar

Car manual built:
Type of car: SportsCar
Count of seats: 2
Engine: volume - 3; mileage - 0
Transmission: SemiAutomatic
GPS Navigator: Functional
```

## Patterns Overview

### Builder Pattern (`src/builder/`)

Separates the construction of a complex object from its representation. The `Director` defines the construction steps, while `Builder` trait allows creating different product representations (Car and Manual) using the same process.

**Key files:**
- `director.rs` - Defines construction sequence
- `builders/mod.rs` - Builder trait
- `builders/car.rs` / `builders/car_manual.rs` - Concrete builders

**To activate in main:** `run_builder()` is called by default.

---

### Factory Method Pattern (`src/factory_method/`)

Defines an interface for creating objects but lets subclasses decide which class to instantiate. `Restaurant` trait creates hamburgers, with `BeefRestaurant` and `ChickenRestaurant` as concrete implementations.

**Key files:**
- `burger.rs` - Hamburger and Restaurant traits
- `beef_hamburger.rs` - Beef implementation
- `init.rs` - Factory selection logic

**To activate in main:** Uncomment `run_factory_method()` in `src/main.rs`.