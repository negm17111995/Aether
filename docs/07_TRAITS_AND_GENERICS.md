# PART 7: TRAITS AND GENERICS

## 7.1 Traits (Interfaces)

### 7.1.1 What are Traits?

Traits define shared behavior. They are like interfaces in other languages.

```aether
trait Printable {
    func to_string(self) -> String
}
```

This says: "Any type that implements `Printable` must have a `to_string` method."

### 7.1.2 Implementing Traits

```aether
struct Point {
    x: Int,
    y: Int,
}

impl Printable for Point {
    func to_string(self) -> String {
        f"Point({self.x}, {self.y})"
    }
}

let p = Point { x: 3, y: 4 }
print(p.to_string())  // Point(3, 4)
```

### 7.1.3 Multiple Traits

A type can implement many traits:

```aether
trait Drawable {
    func draw(self)
}

trait Movable {
    func move_by(mut self, dx: Int, dy: Int)
}

impl Drawable for Point {
    func draw(self) {
        print(f"Drawing point at ({self.x}, {self.y})")
    }
}

impl Movable for Point {
    func move_by(mut self, dx: Int, dy: Int) {
        self.x = self.x + dx
        self.y = self.y + dy
    }
}
```

### 7.1.4 Default Methods

Traits can provide default implementations:

```aether
trait Describable {
    func name(self) -> String
    
    // Default implementation
    func describe(self) -> String {
        f"This is a {self.name()}"
    }
}

struct Dog {
    breed: String,
}

impl Describable for Dog {
    func name(self) -> String {
        self.breed
    }
    // describe() uses default implementation
}

let dog = Dog { breed: "Labrador" }
print(dog.describe())  // This is a Labrador
```

### 7.1.5 Trait Inheritance

```aether
trait Animal {
    func make_sound(self) -> String
}

trait Pet: Animal {
    func name(self) -> String
    
    func introduce(self) -> String {
        f"{self.name()} says {self.make_sound()}"
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    func make_sound(self) -> String {
        "Meow"
    }
}

impl Pet for Cat {
    func name(self) -> String {
        self.name
    }
}
```

---

## 7.2 Common Built-in Traits

### 7.2.1 Clone

```aether
trait Clone {
    func clone(self) -> Self
}

#[derive(Clone)]
struct Point {
    x: Int,
    y: Int,
}

let p1 = Point { x: 10, y: 20 }
let p2 = p1.clone()  // Independent copy
```

### 7.2.2 Eq (Equality)

```aether
trait Eq {
    func eq(self, other: Self) -> Bool
}

#[derive(Eq)]
struct Point { x: Int, y: Int }

let p1 = Point { x: 1, y: 2 }
let p2 = Point { x: 1, y: 2 }
let same = p1 == p2  // true
```

### 7.2.3 Ord (Ordering)

```aether
trait Ord {
    func cmp(self, other: Self) -> Ordering
}

enum Ordering {
    Less,
    Equal,
    Greater,
}

// After implementing Ord, you can use <, >, <=, >=
```

### 7.2.4 Debug

```aether
trait Debug {
    func debug(self) -> String
}

#[derive(Debug)]
struct Point { x: Int, y: Int }

let p = Point { x: 3, y: 4 }
print(f"{p:?}")  // Point { x: 3, y: 4 }
```

### 7.2.5 Default

```aether
trait Default {
    func default() -> Self
}

impl Default for Point {
    func default() -> Point {
        Point { x: 0, y: 0 }
    }
}

let origin = Point::default()
```

---

## 7.3 Generics

### 7.3.1 Generic Functions

```aether
func swap<T>(a: &mut T, b: &mut T) {
    let temp = *a
    *a = *b
    *b = temp
}

let mut x = 5
let mut y = 10
swap(&mut x, &mut y)
// x = 10, y = 5
```

### 7.3.2 Generic Structs

```aether
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    func new(first: T, second: U) -> Pair<T, U> {
        Pair { first, second }
    }
}

let pair = Pair::new(42, "hello")
print(pair.first)   // 42
print(pair.second)  // hello
```

### 7.3.3 Generic Enums

```aether
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

let x: Option<Int> = Some(42)
let y: Result<String, Int> = Ok("success")
```

---

## 7.4 Trait Bounds

### 7.4.1 Basic Bounds

Constrain generic types to implement certain traits:

```aether
// T must implement Printable
func print_item<T: Printable>(item: T) {
    print(item.to_string())
}

// T must implement both Clone and Debug
func clone_and_print<T: Clone + Debug>(item: T) -> T {
    print(item.debug())
    item.clone()
}
```

### 7.4.2 Where Clauses

For complex bounds, use `where`:

```aether
func process<A, B, C>(a: A, b: B, c: C) -> C
where
    A: Clone + Debug,
    B: Printable,
    C: Default + Eq,
{
    // ...
}
```

### 7.4.3 Bounds on Impl Blocks

```aether
struct Container<T> {
    value: T,
}

// Methods for all T
impl<T> Container<T> {
    func new(value: T) -> Container<T> {
        Container { value }
    }
}

// Methods only when T is Printable
impl<T: Printable> Container<T> {
    func print_value(self) {
        print(self.value.to_string())
    }
}

// Methods only when T is Clone
impl<T: Clone> Container<T> {
    func duplicate(self) -> Container<T> {
        Container { value: self.value.clone() }
    }
}
```

---

## 7.5 Associated Types

### 7.5.1 Defining Associated Types

```aether
trait Iterator {
    type Item
    
    func next(mut self) -> Option<Self.Item>
}
```

### 7.5.2 Implementing Associated Types

```aether
struct Counter {
    current: Int,
    max: Int,
}

impl Iterator for Counter {
    type Item = Int
    
    func next(mut self) -> Option<Int> {
        if self.current >= self.max {
            None
        } else {
            let val = self.current
            self.current = self.current + 1
            Some(val)
        }
    }
}
```

---

## 7.6 Trait Objects (Dynamic Dispatch)

### 7.6.1 Trait Object Syntax

```aether
// Box<dyn Trait> - heap-allocated trait object
let drawable: Box<dyn Drawable> = Box::new(point)

// &dyn Trait - reference to trait object
func draw_all(items: [&dyn Drawable]) {
    for item in items {
        item.draw()
    }
}
```

### 7.6.2 When to Use Trait Objects

```aether
struct Circle { radius: Float }
struct Square { side: Float }

impl Drawable for Circle {
    func draw(self) { print("Drawing circle") }
}

impl Drawable for Square {
    func draw(self) { print("Drawing square") }
}

// Heterogeneous collection
let shapes: [Box<dyn Drawable>] = [
    Box::new(Circle { radius: 5.0 }),
    Box::new(Square { side: 10.0 }),
]

for shape in shapes {
    shape.draw()
}
```

### 7.6.3 Static vs Dynamic Dispatch

**Static dispatch (generics):**
```aether
func draw_static<T: Drawable>(item: T) {
    item.draw()  // Resolved at compile time
}
// Faster, but generates more code
```

**Dynamic dispatch (trait objects):**
```aether
func draw_dynamic(item: &dyn Drawable) {
    item.draw()  // Resolved at runtime via vtable
}
// Smaller code, but slight runtime overhead
```

---

## 7.7 Deriving Traits

### 7.7.1 Automatic Implementation

```aether
#[derive(Clone, Debug, Eq)]
struct Point {
    x: Int,
    y: Int,
}

// Now Point has:
// - clone() method
// - debug() method
// - == and != operators
```

### 7.7.2 Available Derive Macros

| Trait | What it provides |
|-------|------------------|
| `Clone` | `.clone()` method |
| `Debug` | Debug formatting |
| `Eq` | `==` and `!=` |
| `Ord` | `<`, `>`, `<=`, `>=` |
| `Hash` | Hashing for use in maps/sets |
| `Default` | `::default()` constructor |

---

*Next: [Part 8: Memory Management](08_MEMORY_MANAGEMENT.md)*
