# PART 6: DATA STRUCTURES

## 6.1 Arrays

### 6.1.1 Creating Arrays

```aether
// Array literal
let numbers = [1, 2, 3, 4, 5]

// Empty array with type
let empty: [Int] = []

// Array with repeated value
let zeros = [0; 10]  // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
```

### 6.1.2 Accessing Elements

```aether
let arr = [10, 20, 30, 40, 50]

let first = arr[0]   // 10
let third = arr[2]   // 30
let last = arr[4]    // 50

// Negative indexing (from end)
let last2 = arr[-1]  // 50
let second_last = arr[-2]  // 40
```

### 6.1.3 Array Length

```aether
let arr = [1, 2, 3, 4, 5]
let len = arr.len()  // 5
let empty = arr.is_empty()  // false
```

### 6.1.4 Modifying Arrays

```aether
let mut arr = [1, 2, 3]

// Modify element
arr[0] = 10
// arr = [10, 2, 3]

// Add element
arr.push(4)
// arr = [10, 2, 3, 4]

// Remove last
let last = arr.pop()
// last = 4, arr = [10, 2, 3]

// Insert at position
arr.insert(1, 15)
// arr = [10, 15, 2, 3]

// Remove at position
let removed = arr.remove(1)
// removed = 15, arr = [10, 2, 3]
```

### 6.1.5 Slicing

```aether
let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]

let slice1 = arr[2..5]    // [2, 3, 4]
let slice2 = arr[..3]     // [0, 1, 2]
let slice3 = arr[7..]     // [7, 8, 9]
let slice4 = arr[2..=5]   // [2, 3, 4, 5]
```

### 6.1.6 Common Array Operations

```aether
let arr = [3, 1, 4, 1, 5, 9, 2, 6]

// Sort
let sorted = arr.sorted()  // [1, 1, 2, 3, 4, 5, 6, 9]

// Reverse
let reversed = arr.reversed()  // [6, 2, 9, 5, 1, 4, 1, 3]

// Contains
let has_five = arr.contains(5)  // true

// Index of
let pos = arr.index_of(4)  // Some(2)

// Join to string
let str = arr.join(", ")  // "3, 1, 4, 1, 5, 9, 2, 6"
```

---

## 6.2 Tuples

### 6.2.1 Creating Tuples

```aether
// Simple tuple
let point = (10, 20)

// Multiple types
let mixed = (42, "hello", true)

// Named for clarity
let person: (String, Int, Bool) = ("Alice", 30, true)
```

### 6.2.2 Accessing Tuple Elements

```aether
let tuple = ("Alice", 30, true)

let name = tuple.0    // "Alice"
let age = tuple.1     // 30
let active = tuple.2  // true
```

### 6.2.3 Destructuring Tuples

```aether
let point = (10, 20)

// Destructure all
let (x, y) = point
// x = 10, y = 20

// Ignore some values
let (x, _) = point
// x = 10, y ignored

// In function parameters
func print_point((x, y): (Int, Int)) {
    print(f"({x}, {y})")
}
```

### 6.2.4 Tuple Returns

```aether
func min_max(arr: [Int]) -> (Int, Int) {
    let mut min = arr[0]
    let mut max = arr[0]
    
    for x in arr {
        if x < min { min = x }
        if x > max { max = x }
    }
    
    (min, max)
}

let (smallest, largest) = min_max([3, 1, 4, 1, 5])
// smallest = 1, largest = 5
```

---

## 6.3 Structs

### 6.3.1 Defining Structs

```aether
struct Point {
    x: Int,
    y: Int,
}

struct Person {
    name: String,
    age: Int,
    email: String,
}
```

### 6.3.2 Creating Instances

```aether
// Named fields
let p = Point { x: 10, y: 20 }

// Field shorthand (when variable name matches)
let x = 10
let y = 20
let p2 = Point { x, y }  // Same as Point { x: x, y: y }

// With some fields from another instance
let p3 = Point { x: 100, ..p }  // y from p
```

### 6.3.3 Accessing Fields

```aether
let person = Person {
    name: "Alice",
    age: 30,
    email: "alice@example.com",
}

print(person.name)   // Alice
print(person.age)    // 30
print(person.email)  // alice@example.com
```

### 6.3.4 Mutable Structs

```aether
let mut person = Person {
    name: "Bob",
    age: 25,
    email: "bob@example.com",
}

person.age = 26
person.email = "bob.new@example.com"
```

### 6.3.5 Methods with `impl`

```aether
struct Rectangle {
    width: Int,
    height: Int,
}

impl Rectangle {
    // Associated function (constructor)
    func new(width: Int, height: Int) -> Rectangle {
        Rectangle { width, height }
    }
    
    // Method (takes self)
    func area(self) -> Int {
        self.width * self.height
    }
    
    func perimeter(self) -> Int {
        2 * (self.width + self.height)
    }
    
    // Mutable method
    func scale(mut self, factor: Int) {
        self.width = self.width * factor
        self.height = self.height * factor
    }
    
    // Returns new instance
    func doubled(self) -> Rectangle {
        Rectangle {
            width: self.width * 2,
            height: self.height * 2,
        }
    }
}

// Usage
let rect = Rectangle::new(10, 20)
let a = rect.area()       // 200
let p = rect.perimeter()  // 60
let big = rect.doubled()  // Rectangle { 20, 40 }
```

### 6.3.6 Tuple Structs

```aether
struct Point3D(Int, Int, Int)
struct Color(u8, u8, u8)
struct Wrapper(Int)

let point = Point3D(1, 2, 3)
let red = Color(255, 0, 0)

print(point.0)  // 1
print(red.0)    // 255
```

### 6.3.7 Unit Structs

```aether
struct Marker

let m = Marker
```

---

## 6.4 Enums

### 6.4.1 Simple Enums

```aether
enum Direction {
    North,
    South,
    East,
    West,
}

let dir = Direction::North

match dir {
    Direction::North => print("Going north"),
    Direction::South => print("Going south"),
    Direction::East => print("Going east"),
    Direction::West => print("Going west"),
}
```

### 6.4.2 Enums with Values

```aether
enum Color {
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8),
    Hex(String),
}

let c1 = Color::Red
let c2 = Color::RGB(255, 128, 0)
let c3 = Color::Hex("#FF8000")

match c2 {
    Color::RGB(r, g, b) => print(f"R={r}, G={g}, B={b}"),
    _ => print("Other color"),
}
```

### 6.4.3 Enums with Named Fields

```aether
enum Message {
    Quit,
    Move { x: Int, y: Int },
    Write(String),
    ChangeColor { r: u8, g: u8, b: u8 },
}

let msg = Message::Move { x: 10, y: 20 }

match msg {
    Message::Quit => print("Quit"),
    Message::Move { x, y } => print(f"Move to ({x}, {y})"),
    Message::Write(text) => print(f"Write: {text}"),
    Message::ChangeColor { r, g, b } => print(f"Color: {r},{g},{b}"),
}
```

### 6.4.4 Enum Methods

```aether
impl Direction {
    func opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
    
    func is_vertical(self) -> Bool {
        match self {
            Direction::North | Direction::South => true,
            _ => false,
        }
    }
}

let dir = Direction::North
let opp = dir.opposite()  // Direction::South
let vert = dir.is_vertical()  // true
```

---

## 6.5 Option Type

### 6.5.1 What is Option?

`Option` represents a value that might not exist:

```aether
enum Option<T> {
    Some(T),   // Value exists
    None,      // No value
}
```

### 6.5.2 Using Option

```aether
func find(arr: [Int], target: Int) -> Option<Int> {
    for i in 0..arr.len() {
        if arr[i] == target {
            return Some(i)
        }
    }
    None
}

let numbers = [10, 20, 30, 40, 50]

match find(numbers, 30) {
    Some(index) => print(f"Found at index {index}"),
    None => print("Not found"),
}
```

### 6.5.3 Option Methods

```aether
let opt = Some(42)

// Check if has value
let has = opt.is_some()  // true
let empty = opt.is_none()  // false

// Get value (panics if None)
let val = opt.unwrap()  // 42

// Get with default
let val2 = opt.unwrap_or(0)  // 42
let val3 = None.unwrap_or(0)  // 0

// Map the inner value
let doubled = opt.map(|x| x * 2)  // Some(84)

// Chain operations
let result = Some(5)
    .map(|x| x * 2)
    .map(|x| x + 1)
    .unwrap_or(0)
// 11
```

---

## 6.6 Result Type

### 6.6.1 What is Result?

`Result` represents success or failure:

```aether
enum Result<T, E> {
    Ok(T),     // Success
    Err(E),    // Failure
}
```

### 6.6.2 Using Result

```aether
func divide(a: Int, b: Int) -> Result<Int, String> {
    if b == 0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}

match divide(10, 2) {
    Ok(result) => print(f"Result: {result}"),
    Err(msg) => print(f"Error: {msg}"),
}
```

### 6.6.3 Result Methods

```aether
let ok: Result<Int, String> = Ok(42)
let err: Result<Int, String> = Err("failed")

// Check status
ok.is_ok()   // true
ok.is_err()  // false

// Get value
ok.unwrap()       // 42
err.unwrap_or(0)  // 0

// Map success value
ok.map(|x| x * 2)  // Ok(84)

// Map error value
err.map_err(|e| f"ERROR: {e}")  // Err("ERROR: failed")

// Chain operations
let result = Ok(5)
    .map(|x| x * 2)
    .and_then(|x| if x > 5 { Ok(x) } else { Err("too small") })
// Ok(10)
```

---

*Next: [Part 7: Traits and Generics](07_TRAITS_AND_GENERICS.md)*
