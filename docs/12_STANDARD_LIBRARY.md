# PART 12: STANDARD LIBRARY

## 12.1 Overview

The Aether standard library provides:
- Core types and traits
- Collections (Vec, HashMap, etc.)
- I/O operations
- File system access
- Networking
- Concurrency primitives
- Time and date
- String manipulation
- Math operations

---

## 12.2 Collections

### 12.2.1 Vec (Dynamic Array)

```aether
use std::collections::Vec

// Create
let mut v = Vec::new()
let v2 = vec![1, 2, 3, 4, 5]

// Add elements
v.push(10)
v.push(20)
v.push(30)

// Access
let first = v[0]           // 10
let last = v[v.len() - 1]  // 30
let maybe = v.get(5)       // None

// Modify
v[0] = 100

// Remove
let popped = v.pop()       // Some(30)
let removed = v.remove(0)  // 100

// Iterate
for item in v {
    print(item)
}

// Length
v.len()       // Current length
v.capacity()  // Allocated capacity
v.is_empty()  // Check if empty

// Transform
let doubled = v.iter().map(|x| x * 2).collect()
let evens = v.iter().filter(|x| x % 2 == 0).collect()
```

### 12.2.2 HashMap (Key-Value Store)

```aether
use std::collections::HashMap

// Create
let mut map = HashMap::new()

// Insert
map.insert("name", "Alice")
map.insert("city", "Paris")

// Access
let name = map.get("name")  // Some("Alice")
let age = map.get("age")    // None

// Check existence
map.contains_key("name")  // true

// Remove
map.remove("city")

// Iterate
for (key, value) in map {
    print(f"{key}: {value}")
}

// Entry API
map.entry("count")
    .or_insert(0)
    .and_modify(|c| *c += 1)
```

### 12.2.3 HashSet (Unique Values)

```aether
use std::collections::HashSet

// Create
let mut set = HashSet::new()

// Add
set.insert(1)
set.insert(2)
set.insert(3)
set.insert(2)  // Ignored (duplicate)

// Check
set.contains(2)  // true
set.len()        // 3

// Remove
set.remove(1)

// Set operations
let a = hashset![1, 2, 3]
let b = hashset![2, 3, 4]

let union = a.union(&b)         // {1, 2, 3, 4}
let intersection = a.intersection(&b)  // {2, 3}
let difference = a.difference(&b)      // {1}
```

### 12.2.4 VecDeque (Double-Ended Queue)

```aether
use std::collections::VecDeque

let mut deque = VecDeque::new()

// Add to front/back
deque.push_back(1)
deque.push_back(2)
deque.push_front(0)  // [0, 1, 2]

// Remove from front/back
deque.pop_front()  // Some(0)
deque.pop_back()   // Some(2)
```

### 12.2.5 BinaryHeap (Priority Queue)

```aether
use std::collections::BinaryHeap

let mut heap = BinaryHeap::new()

heap.push(3)
heap.push(1)
heap.push(4)
heap.push(1)
heap.push(5)

// Always pops largest
heap.pop()  // Some(5)
heap.pop()  // Some(4)
heap.pop()  // Some(3)
```

---

## 12.3 String Operations

### 12.3.1 String Type

```aether
// Create
let s1 = "Hello"
let s2 = String::from("World")
let s3 = String::new()

// Concatenate
let combined = s1 + " " + s2    // "Hello World"
let formatted = f"{s1}, {s2}!"  // "Hello, World!"

// Length
s1.len()       // 5 (bytes)
s1.chars().len()  // 5 (characters)

// Check
s1.is_empty()           // false
s1.starts_with("He")    // true
s1.ends_with("lo")      // true
s1.contains("ell")      // true
```

### 12.3.2 String Manipulation

```aether
let s = "  Hello, World!  "

// Trim whitespace
s.trim()        // "Hello, World!"
s.trim_start()  // "Hello, World!  "
s.trim_end()    // "  Hello, World!"

// Case conversion
s.to_uppercase()  // "  HELLO, WORLD!  "
s.to_lowercase()  // "  hello, world!  "

// Replace
s.replace("World", "Aether")  // "  Hello, Aether!  "

// Split
let parts = "a,b,c".split(",")  // ["a", "b", "c"]
let words = "hello world".split_whitespace()  // ["hello", "world"]

// Slice
let hello = &s[2..7]  // "Hello"
```

### 12.3.3 Parsing

```aether
// String to number
let n: Int = "42".parse().unwrap()
let f: Float = "3.14".parse().unwrap()

// Number to string
let s = 42.to_string()  // "42"
let s = format!("{}", 3.14)  // "3.14"
```

---

## 12.4 I/O Operations

### 12.4.1 Console I/O

```aether
use std::io

// Output
print("Hello")        // No newline
println("World")      // With newline
eprint("Error!")      // To stderr
eprintln("Error!")    // To stderr with newline

// Input
let line = io::read_line()
print(f"You typed: {line}")

// Formatted print
let name = "Alice"
let age = 30
println(f"Name: {name}, Age: {age}")
```

### 12.4.2 File Reading

```aether
use std::fs

// Read entire file
let content = fs::read_to_string("file.txt")?

// Read bytes
let bytes = fs::read("file.bin")?

// Read lines
for line in fs::read_lines("file.txt")? {
    println(line)
}
```

### 12.4.3 File Writing

```aether
use std::fs

// Write string
fs::write("output.txt", "Hello, World!")?

// Write bytes
fs::write("output.bin", &[0, 1, 2, 3])?

// Append
let mut file = fs::OpenOptions::new()
    .append(true)
    .open("log.txt")?
file.write("New log entry\n")?
```

### 12.4.4 File Operations

```aether
use std::fs

// Check existence
fs::exists("myfile.txt")  // true/false

// Create directory
fs::create_dir("newdir")?
fs::create_dir_all("path/to/nested/dir")?

// Remove
fs::remove_file("myfile.txt")?
fs::remove_dir("emptydir")?
fs::remove_dir_all("dir_with_contents")?

// Copy and move
fs::copy("src.txt", "dst.txt")?
fs::rename("old.txt", "new.txt")?

// Metadata
let meta = fs::metadata("file.txt")?
meta.len()          // File size in bytes
meta.is_file()      // true
meta.is_dir()       // false
meta.modified()     // Modification time
```

---

## 12.5 Networking

### 12.5.1 HTTP Client

```aether
use std::net::http

// GET request
let response = http::get("https://api.example.com/data")?
println(response.body)

// POST request
let response = http::post("https://api.example.com/submit")
    .header("Content-Type", "application/json")
    .body(r#"{"name": "Alice"}"#)
    .send()?

// Response handling
response.status      // 200
response.headers     // HashMap
response.body        // String
```

### 12.5.2 HTTP Server

```aether
use std::net::http

http::serve(8080, |request| {
    match request.path {
        "/" => http::Response::ok("Hello, World!"),
        "/api/data" => {
            let data = r#"{"status": "ok"}"#
            http::Response::json(data)
        },
        _ => http::Response::not_found(),
    }
})
```

### 12.5.3 TCP Sockets

```aether
use std::net::tcp

// Client
let mut stream = tcp::connect("127.0.0.1:8080")?
stream.write("Hello, Server!")?
let response = stream.read()?

// Server
let listener = tcp::bind("127.0.0.1:8080")?
for stream in listener.incoming() {
    spawn(|| handle_client(stream))
}
```

---

## 12.6 Time and Date

### 12.6.1 Duration

```aether
use std::time::Duration

let d1 = Duration::seconds(30)
let d2 = Duration::millis(500)
let d3 = Duration::micros(1000)

let total = d1 + d2  // 30.5 seconds
```

### 12.6.2 Instant (Timing)

```aether
use std::time::Instant

let start = Instant::now()

// Do work...

let elapsed = start.elapsed()
println(f"Took {elapsed.millis()}ms")
```

### 12.6.3 Date and Time

```aether
use std::time::{Date, Time, DateTime}

// Current time
let now = DateTime::now()
println(now.format("%Y-%m-%d %H:%M:%S"))

// Parse
let dt = DateTime::parse("2024-01-15 10:30:00")?

// Components
dt.year()    // 2024
dt.month()   // 1
dt.day()     // 15
dt.hour()    // 10
dt.minute()  // 30
```

### 12.6.4 Sleep

```aether
use std::thread
use std::time::Duration

thread::sleep(Duration::seconds(1))
```

---

## 12.7 Math

### 12.7.1 Basic Math

```aether
use std::math

// Constants
math::PI        // 3.14159...
math::E         // 2.71828...
math::INFINITY  // +∞

// Functions
math::abs(-5)       // 5
math::min(3, 7)     // 3
math::max(3, 7)     // 7
math::clamp(5, 0, 3)  // 3 (clamp between 0 and 3)
```

### 12.7.2 Floating Point

```aether
math::sqrt(16.0)    // 4.0
math::pow(2.0, 10.0)  // 1024.0
math::log(100.0)    // 4.605...
math::log10(100.0)  // 2.0
math::exp(1.0)      // 2.718...

math::floor(3.7)    // 3.0
math::ceil(3.2)     // 4.0
math::round(3.5)    // 4.0
```

### 12.7.3 Trigonometry

```aether
math::sin(0.0)      // 0.0
math::cos(0.0)      // 1.0
math::tan(0.0)      // 0.0

math::asin(0.5)     // 0.523...
math::acos(0.5)     // 1.047...
math::atan(1.0)     // 0.785...

math::atan2(1.0, 1.0)  // 0.785... (angle from x-axis)
```

### 12.7.4 Random Numbers

```aether
use std::random

random::int()           // Random Int
random::range(1, 100)   // Random 1-100
random::float()         // Random 0.0-1.0
random::bool()          // Random true/false
random::choice([1,2,3]) // Random element
random::shuffle(&mut arr)  // Shuffle array
```

---

*Next: [Part 13: Advanced Topics](13_ADVANCED.md)*
