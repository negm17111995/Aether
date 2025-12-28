# PART 10: ERROR HANDLING

## 10.1 Philosophy

### 10.1.1 Errors are Values

Aether treats errors as regular values, not exceptions:

- No hidden control flow
- Errors must be handled explicitly
- Compiler enforces error handling

### 10.1.2 Two Types of Errors

| Type | Use When | Mechanism |
|------|----------|-----------|
| **Recoverable** | Expected failures | `Result<T, E>` |
| **Unrecoverable** | Bugs, impossible states | `panic()` |

---

## 10.2 Result Type

### 10.2.1 Definition

```aether
enum Result<T, E> {
    Ok(T),     // Success with value T
    Err(E),    // Failure with error E
}
```

### 10.2.2 Creating Results

```aether
// Success
let good: Result<Int, String> = Ok(42)

// Failure
let bad: Result<Int, String> = Err("something went wrong")
```

### 10.2.3 Checking Results

```aether
let result = divide(10, 2)

if result.is_ok() {
    print("Success!")
}

if result.is_err() {
    print("Failed!")
}
```

### 10.2.4 Pattern Matching

```aether
func divide(a: Int, b: Int) -> Result<Int, String> {
    if b == 0 {
        Err("division by zero")
    } else {
        Ok(a / b)
    }
}

let result = divide(10, 2)

match result {
    Ok(value) => print(f"Result: {value}"),
    Err(msg) => print(f"Error: {msg}"),
}
```

### 10.2.5 Unwrapping

```aether
let result: Result<Int, String> = Ok(42)

// Panics if Err
let value = result.unwrap()  // 42

// With custom panic message
let value = result.expect("should have value")

// With default value
let value = result.unwrap_or(0)

// With computed default
let value = result.unwrap_or_else(|| compute_default())
```

### 10.2.6 Transforming Results

```aether
let result: Result<Int, String> = Ok(5)

// Map success value
let doubled = result.map(|x| x * 2)  // Ok(10)

// Map error value
let result2: Result<Int, String> = Err("oops")
let better_err = result2.map_err(|e| f"Error: {e}")
// Err("Error: oops")

// Chain operations
let final_result = Ok(5)
    .map(|x| x * 2)       // Ok(10)
    .map(|x| x + 1)       // Ok(11)
// Ok(11)
```

---

## 10.3 Error Propagation

### 10.3.1 Manual Propagation

```aether
func read_config() -> Result<String, IoError> {
    let file = open_file("config.txt")
    match file {
        Ok(f) => {
            match f.read_all() {
                Ok(content) => Ok(content),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}
```

### 10.3.2 The ? Operator Pattern

Aether supports early return for errors:

```aether
func read_config() -> Result<String, IoError> {
    let file = try_unwrap(open_file("config.txt"))
    if try_failed(file) {
        return Err(file)
    }
    
    let content = try_unwrap(file.read_all())
    if try_failed(content) {
        return Err(content)
    }
    
    Ok(content)
}
```

### 10.3.3 Chaining with and_then

```aether
func process_config() -> Result<Config, Error> {
    read_file("config.txt")
        .and_then(|content| parse_config(content))
        .and_then(|config| validate(config))
        .map(|config| apply_defaults(config))
}
```

---

## 10.4 Option Type

### 10.4.1 Definition

```aether
enum Option<T> {
    Some(T),  // Has value
    None,     // No value
}
```

### 10.4.2 Usage

```aether
func find_user(id: Int) -> Option<User> {
    if let Some(user) = database.get(id) {
        Some(user)
    } else {
        None
    }
}

match find_user(42) {
    Some(user) => print(f"Found: {user.name}"),
    None => print("User not found"),
}
```

### 10.4.3 Option Methods

```aether
let opt: Option<Int> = Some(42)

// Check
opt.is_some()  // true
opt.is_none()  // false

// Unwrap
opt.unwrap()        // 42 (panics if None)
opt.unwrap_or(0)    // 42 (returns 0 if None)
opt.expect("msg")   // 42 (panics with msg if None)

// Transform
opt.map(|x| x * 2)  // Some(84)

// Chain
opt.and_then(|x| if x > 0 { Some(x) } else { None })

// Convert to Result
opt.ok_or("no value")  // Ok(42) or Err("no value")
```

### 10.4.4 Option vs Null

**In other languages:**
```javascript
// JavaScript
let user = findUser(42);
if (user !== null) {
    console.log(user.name);  // Could crash if null check forgotten
}
```

**In Aether:**
```aether
// Aether - compiler forces you to handle None
let user = find_user(42)
match user {
    Some(u) => print(u.name),  // Safe!
    None => print("not found"),
}
```

---

## 10.5 Custom Error Types

### 10.5.1 Error Enum

```aether
enum ConfigError {
    FileNotFound(String),
    ParseError { line: Int, message: String },
    ValidationError(String),
}

func load_config(path: String) -> Result<Config, ConfigError> {
    if !file_exists(path) {
        return Err(ConfigError::FileNotFound(path))
    }
    
    let content = read_file(path).unwrap()
    
    match parse(content) {
        Ok(config) => Ok(config),
        Err(e) => Err(ConfigError::ParseError {
            line: e.line,
            message: e.msg,
        }),
    }
}
```

### 10.5.2 Error Trait

```aether
trait Error {
    func message(self) -> String
    func source(self) -> Option<&dyn Error>
}

impl Error for ConfigError {
    func message(self) -> String {
        match self {
            ConfigError::FileNotFound(path) => 
                f"Config file not found: {path}",
            ConfigError::ParseError { line, message } => 
                f"Parse error at line {line}: {message}",
            ConfigError::ValidationError(msg) => 
                f"Validation failed: {msg}",
        }
    }
    
    func source(self) -> Option<&dyn Error> {
        None
    }
}
```

### 10.5.3 Error Conversion

```aether
impl From<IoError> for ConfigError {
    func from(e: IoError) -> ConfigError {
        ConfigError::FileNotFound(e.path)
    }
}

// Now IoError automatically converts to ConfigError
func load(path: &str) -> Result<Config, ConfigError> {
    let content = read_file(path)?  // IoError -> ConfigError
    parse(content)
}
```

---

## 10.6 Panic

### 10.6.1 When to Panic

Panic for **unrecoverable** errors:
- Programming bugs
- Impossible states
- Violated invariants

```aether
func get(arr: [Int], index: Int) -> Int {
    if index >= arr.len() {
        panic("index out of bounds")
    }
    arr[index]
}
```

### 10.6.2 Panic vs Result

| Use `panic` when: | Use `Result` when: |
|-------------------|-------------------|
| Bug in code | Expected failure |
| Invariant violated | User input error |
| Impossible state | Resource unavailable |
| Tests/prototyping | Recoverable situation |

### 10.6.3 Assert

```aether
// Panics if condition is false
assert(x > 0)
assert(x > 0, "x must be positive")

// Debug-only assert (removed in release)
debug_assert(expensive_check())
```

---

## 10.7 Error Handling Patterns

### 10.7.1 Early Return Pattern

```aether
func process(input: String) -> Result<Output, Error> {
    // Validate early
    if input.is_empty() {
        return Err(Error::InvalidInput)
    }
    
    // Process
    let parsed = try_unwrap(parse(input))
    if try_failed(parsed) {
        return Err(parsed)
    }
    
    // Transform
    let result = transform(parsed)
    
    Ok(result)
}
```

### 10.7.2 Default Values Pattern

```aether
// Use default on error
let config = load_config("app.conf").unwrap_or(Config::default())

// Compute default on error
let value = parse_int(input).unwrap_or_else(|| {
    log("parse failed, using default")
    0
})
```

### 10.7.3 Collect Results Pattern

```aether
let inputs = ["1", "2", "abc", "4"]

// Collect all successes (ignore errors)
let valid: [Int] = inputs
    .filter_map(|s| parse_int(s).ok())
// [1, 2, 4]

// Fail on first error
let all: Result<[Int], Error> = inputs
    .map(parse_int)
    .collect()
// Err(...) because "abc" failed
```

### 10.7.4 Context Pattern

```aether
func load_user(id: Int) -> Result<User, Error> {
    let file = read_file(f"/users/{id}.json")
        .map_err(|e| Error::new(f"failed to load user {id}", e))?
    
    let user = parse_user(file)
        .map_err(|e| Error::new(f"failed to parse user {id}", e))?
    
    Ok(user)
}
```

---

*Next: [Part 11: Modules and Imports](11_MODULES.md)*
