# Rust Design Patterns

A practical demo project showcasing common design patterns implemented in Rust.
---

Pattern Classification:
- Creational: How objects are created
  - Abstract Factory, Builder, Factory Method, Prototype
- Structural: How objects are composed
  - Adapter
- Behavioral: How objects interact
  - (Not yet implemented)

---
## Patterns Included

| Pattern              | Category   | Description                                                                                                              | Module                  |
| -------------------- | ---------- | ------------------------------------------------------------------------------------------------------------------------ | ----------------------- |
| **Abstract Factory** | Creational | Provides an interface for creating families of related objects without specifying their concrete classes.                | `src/abstract_factory/` |
| **Adapter**          | Structural | Converts the interface of a class into another interface that clients expect.                                            | `src/adapter/`          |
| **Builder**          | Creational | Separates object construction from representation, allowing different representations for the same construction process. | `src/builder/`          |
| **Factory Method**   | Creational | Defines an interface for creating objects, letting subclasses decide which class to instantiate.                         | `src/factory_method/`   |
| **Prototype**        | Creational | Creates new objects by copying an existing object (prototype).                                                           | `src/prototype/`        |

## Project Structure

```
design_patterns/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── abstract_factory/
    │   ├── mod.rs             # Static & dynamic dispatch examples
    │   ├── restaurant_factory.rs  # Traits with docs
    │   ├── burger.rs          # Hamburger trait
    │   ├── drink.rs          # Drink trait
    │   ├── fast_food/        # Fast food factory (BeefHamburger, Soda)
    │   └── healthy/          # Healthy factory (ChickenHamburger, Water)
    ├── adapter/
    │   ├── mod.rs
    │   ├── target.rs         # Target interface
    │   └── adaptee.rs        # Adaptee implementation
    ├── builder/
    │   ├── mod.rs
    │   ├── components.rs     # Shared components
    │   ├── director.rs       # Director for construction steps
    │   ├── builders/
    │   │   ├── mod.rs        # Builder trait
    │   │   ├── car.rs        # CarBuilder
    │   │   └── car_manual.rs # CarManualBuilder
    │   └── cars/
    │       ├── mod.rs
    │       ├── car.rs        # Car product
    │       └── manual.rs     # Manual product
    ├── factory_method/
    │   ├── mod.rs
    │   ├── burger.rs         # Traits: Hamburger, Restaurant
    │   ├── beef_hamburger.rs # BeefHamburger + BeefRestaurant
    │   ├── chicken_hamburger.rs
    │   └── init.rs           # Factory initialization
    └── prototype/
        ├── mod.rs
        └── document.rs       # Document (cloneable)
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed

## Execution

```bash
cargo build
cargo run
```
## Patterns Overview

### Abstract Factory Pattern (`src/abstract_factory/`)

**Description:** Provides an interface for creating families of related objects without specifying their concrete classes. This implementation demonstrates both static and dynamic dispatch approaches.

**Key files:**
- `restaurant_factory.rs` - RestaurantFactory and RestaurantFactoryDyn traits
- `mod.rs` - `order_with_static()` and `order_with_dynamic()` examples
- `healthy/` - HealthyRestaurant (ChickenHamburger, Water)
- `fast_food/` - FastFoodRestaurant (BeefHamburger, Soda)

**Example:**
```
¿Qué menú quieres? (1. saludable/ 2. comida rápida)
1
Preparando una hamburguesa de pollo.
Sirviendo un vaso con agua.
```

---

### Adapter Pattern (`src/adapter/`)

**Description:** Converts the interface of a class into another interface that clients expect. Allows incompatible interfaces to work together.

**Key files:**
- `target.rs` - Target interface that clients use
- `adaptee.rs` - Existing interface that needs adapting
- `mod.rs` - TargetAdapter implementation

**Example:**
```
A compatible target can be directly called: 'Hello, World!'
Adaptee is incompatible with client: 'Hello, World!'
But with adapter client can call its method: '!dlroW ,olleH'
```

---

### Builder Pattern (`src/builder/`)

**Description:** Separates the construction of a complex object from its representation. The Director defines construction steps while the Builder trait allows creating different product representations.

**Key files:**
- `director.rs` - Defines construction sequence
- `builders/mod.rs` - Builder trait
- `builders/car.rs` / `builders/car_manual.rs` - Concrete builders
- `cars/car.rs` / `cars/manual.rs` - Product types

**Example:**
```
Car built: SportsCar

Car manual built:
Type of car: SportsCar
Count of seats: 2
Engine: volume - 3; mileage - 0
Transmission: SemiAutomatic
GPS Navigator: Functional
```

---

### Factory Method Pattern (`src/factory_method/`)

**Description:** Defines an interface for creating objects but lets subclasses decide which class to instantiate.

**Key files:**
- `burger.rs` - Hamburger and Restaurant traits
- `beef_hamburger.rs` - Beef implementation
- `chicken_hamburger.rs` - Chicken implementation
- `init.rs` - Factory initialization logic

**Example:**
```
Creating restaurant...
Preparing a beef hamburger.
```

---

### Prototype Pattern (`src/prototype/`)

**Description:** Creates new objects by copying an existing object (prototype). Useful when creating objects is expensive or complex.

**Key files:**
- `document.rs` - Document struct implementing Clone trait

**Example:**
```
Document 1: Document { title: "Computer quote", content: "$2800", author: "Apple" }
Document 2: Document { title: "Computer quote", content: "$2800", author: "Lenovo" }
```

---

## To Activate

Select the corresponding option from the main menu (1-6).