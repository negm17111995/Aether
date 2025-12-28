# PART 11: MODULES AND IMPORTS

## 11.1 Module System Overview

### 11.1.1 What are Modules?

Modules organize code into logical units:
- Group related functionality
- Control visibility (public/private)
- Avoid naming conflicts
- Enable code reuse

### 11.1.2 Module Hierarchy

```
project/
├── main.aether          # Entry point
├── lib.aether           # Library root
├── utils/
│   ├── mod.aether       # Module definition
│   ├── string.aether    # Submodule
│   └── math.aether      # Submodule
└── models/
    ├── mod.aether
    ├── user.aether
    └── product.aether
```

---

## 11.2 Creating Modules

### 11.2.1 Inline Modules

```aether
// In main.aether

mod math {
    pub func add(a: Int, b: Int) -> Int {
        a + b
    }
    
    pub func multiply(a: Int, b: Int) -> Int {
        a * b
    }
    
    // Private function (not accessible outside)
    func internal_helper() -> Int {
        42
    }
}

// Usage
let sum = math::add(2, 3)
```

### 11.2.2 File Modules

```aether
// utils/math.aether

pub func add(a: Int, b: Int) -> Int {
    a + b
}

pub func subtract(a: Int, b: Int) -> Int {
    a - b
}
```

```aether
// main.aether

mod utils::math  // Loads utils/math.aether

func main() {
    let x = utils::math::add(5, 3)
}
```

### 11.2.3 Directory Modules

```aether
// utils/mod.aether (defines the utils module)

pub mod math    // Re-export utils/math.aether
pub mod string  // Re-export utils/string.aether

// Can also define items here
pub const VERSION: Int = 1
```

---

## 11.3 Imports with `use`

### 11.3.1 Basic Import

```aether
use std::io

func main() {
    io::print("Hello")
    io::println("World")
}
```

### 11.3.2 Import Specific Items

```aether
use std::io::{print, println}

func main() {
    print("Hello")
    println("World")
}
```

### 11.3.3 Import All

```aether
use std::math::*

func main() {
    let x = sin(0.5)
    let y = cos(0.5)
    let z = sqrt(2.0)
}
```

### 11.3.4 Aliased Import

```aether
use std::collections::HashMap as Map

func main() {
    let m = Map::new()
}
```

### 11.3.5 Nested Imports

```aether
use std::{
    io::{print, println},
    collections::{Vec, HashMap},
    fs::File,
}
```

---

## 11.4 Visibility

### 11.4.1 Public vs Private

```aether
mod example {
    // Private (default) - only visible in this module
    func private_helper() -> Int {
        42
    }
    
    // Public - visible everywhere
    pub func public_function() -> Int {
        private_helper()  // Can use private
    }
    
    pub struct Point {
        pub x: Int,    // Public field
        pub y: Int,    // Public field
        secret: Int,   // Private field
    }
}

// Outside module:
example::public_function()  // OK
example::private_helper()   // ERROR: private
```

### 11.4.2 pub(crate) - Crate-Only Visibility

```aether
pub(crate) func internal_api() {
    // Visible within this crate only
}
```

### 11.4.3 pub(super) - Parent Module Only

```aether
mod parent {
    mod child {
        pub(super) func helper() {
            // Only visible to parent module
        }
    }
    
    func use_helper() {
        child::helper()  // OK
    }
}
// parent::child::helper()  // ERROR
```

---

## 11.5 Re-exports

### 11.5.1 Basic Re-export

```aether
// lib.aether
mod internal {
    pub struct Config { ... }
    pub func load() -> Config { ... }
}

// Re-export for easier access
pub use internal::Config
pub use internal::load
```

Now users can:
```aether
use mylib::Config  // Instead of mylib::internal::Config
```

### 11.5.2 Rename Re-export

```aether
pub use internal::Config as AppConfig
```

### 11.5.3 Aggregate Re-exports

```aether
// prelude.aether - common imports
pub use std::io::{print, println}
pub use std::collections::Vec
pub use std::result::Result
pub use std::option::Option
```

```aether
// User code
use mylib::prelude::*
```

---

## 11.6 Standard Library Structure

### 11.6.1 Core Modules

```aether
use std::core        // Basic types: Int, String, Bool
use std::collections // Vec, HashMap, HashSet
use std::io          // Input/Output
use std::fs          // File system
use std::net         // Networking
use std::thread      // Threading
use std::sync        // Synchronization primitives
use std::time        // Time and duration
```

### 11.6.2 Core Types (Auto-imported)

These are available without import:
- `Int`, `Float`, `Bool`, `String`, `Char`
- `Option<T>`, `Some`, `None`
- `Result<T,E>`, `Ok`, `Err`
- `Vec<T>`

### 11.6.3 Prelude

```aether
// Automatically imported
use std::prelude::*
```

Contains:
- Basic traits: `Clone`, `Debug`, `Eq`, `Ord`
- IO: `print`, `println`
- Memory: `Box`
- Common functions

---

## 11.7 Path Resolution

### 11.7.1 Absolute Paths

Start from crate root:

```aether
use crate::utils::math
use std::collections::HashMap
```

### 11.7.2 Relative Paths

Start from current module:

```aether
// In utils/string.aether
use super::math  // Parent: utils::math
use self::helpers  // Current module's helpers
```

### 11.7.3 Path Keywords

| Keyword | Meaning |
|---------|---------|
| `crate` | Root of current crate |
| `self` | Current module |
| `super` | Parent module |
| `std` | Standard library |

---

## 11.8 Packages and Crates

### 11.8.1 What is a Crate?

A crate is a compilation unit:
- Library crate: `lib.aether`
- Binary crate: `main.aether`

### 11.8.2 What is a Package?

A package contains:
- One or more crates
- A `Aether.toml` configuration file

```toml
# Aether.toml
[package]
name = "myproject"
version = "1.0.0"

[dependencies]
json = "2.0"
http = "1.5"
```

### 11.8.3 Using External Packages

```aether
// After adding to Aether.toml
use json::parse
use http::Client

func main() {
    let data = parse(r#"{"name": "Alice"}"#)
    let client = Client::new()
}
```

---

## 11.9 Best Practices

### 11.9.1 Module Organization

```
src/
├── main.aether        # Entry point, minimal code
├── lib.aether         # Library interface
├── config/
│   ├── mod.aether     # Config module
│   └── parser.aether  # Config parsing
├── models/
│   ├── mod.aether
│   ├── user.aether
│   └── product.aether
├── handlers/
│   ├── mod.aether
│   ├── auth.aether
│   └── api.aether
└── utils/
    ├── mod.aether
    └── helpers.aether
```

### 11.9.2 Import Style

```aether
// Group imports by source
use std::io::{print, println}
use std::collections::HashMap

use crate::config::Config
use crate::models::{User, Product}

use external_lib::Client
```

### 11.9.3 Visibility Guidelines

- Make as little public as possible
- Use `pub(crate)` for internal APIs
- Document all public items
- Re-export at crate root for convenience

---

*Next: [Part 12: Standard Library](12_STANDARD_LIBRARY.md)*
