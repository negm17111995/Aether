# PART 14: BEST PRACTICES

## 14.1 Code Style

### 14.1.1 Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Variables | `snake_case` | `user_name`, `total_count` |
| Functions | `snake_case` | `get_user`, `calculate_total` |
| Types | `PascalCase` | `UserProfile`, `HttpClient` |
| Constants | `SCREAMING_SNAKE` | `MAX_SIZE`, `DEFAULT_PORT` |
| Modules | `snake_case` | `user_auth`, `http_client` |
| Type params | Single uppercase | `T`, `E`, `K`, `V` |

### 14.1.2 File Organization

```aether
// 1. Module documentation
//! This module handles user authentication.

// 2. Imports (grouped)
use std::io
use std::collections::HashMap

use crate::config
use crate::models::User

// 3. Constants
const MAX_ATTEMPTS: Int = 3

// 4. Type definitions
struct AuthResult {
    success: Bool,
    user: Option<User>,
}

enum AuthError {
    InvalidCredentials,
    AccountLocked,
}

// 5. Traits
trait Authenticator {
    func authenticate(self, creds: Credentials) -> Result<User, AuthError>
}

// 6. Implementations
impl Authenticator for BasicAuth {
    // ...
}

// 7. Functions
func verify_password(hash: String, password: String) -> Bool {
    // ...
}

// 8. Tests
#[cfg(test)]
mod tests {
    // ...
}
```

### 14.1.3 Line Length

Keep lines under 100 characters:

```aether
// Bad: Too long
let result = very_long_function_name(first_argument, second_argument, third_argument, fourth_argument)

// Good: Break it up
let result = very_long_function_name(
    first_argument,
    second_argument,
    third_argument,
    fourth_argument,
)
```

### 14.1.4 Indentation

Use 4 spaces (not tabs):

```aether
func example() {
    if condition {
        do_something()
    } else {
        do_other()
    }
}
```

---

## 14.2 Error Handling

### 14.2.1 Use Result for Expected Errors

```aether
// Good: Returns Result for expected failure
func parse_config(path: &str) -> Result<Config, ConfigError> {
    let content = read_file(path)?
    parse_json(content)
}

// Bad: Panics on expected failure
func parse_config_bad(path: &str) -> Config {
    let content = read_file(path).unwrap()  // Don't do this!
    parse_json(content).unwrap()
}
```

### 14.2.2 Provide Context

```aether
// Bad: No context
func load_user(id: Int) -> Result<User, Error> {
    database.get(id)?
}

// Good: Add context
func load_user(id: Int) -> Result<User, Error> {
    database.get(id)
        .map_err(|e| Error::new(f"failed to load user {id}", e))
}
```

### 14.2.3 Use Type-Specific Errors

```aether
// Bad: String errors
func process() -> Result<Data, String> {
    Err("something went wrong")
}

// Good: Typed errors
enum ProcessError {
    InvalidInput(String),
    NetworkFailure(IoError),
    ParseError { line: Int, message: String },
}

func process() -> Result<Data, ProcessError> {
    Err(ProcessError::InvalidInput("missing field: name".into()))
}
```

### 14.2.4 Don't Silence Errors

```aether
// Bad: Error ignored
func bad_example() {
    let _ = try_something()  // Error disappears
}

// Good: Handle or propagate
func good_example() -> Result<(), Error> {
    try_something()?  // Propagate
    Ok(())
}

// Or explicitly ignore with reason
func intentional_ignore() {
    // We don't care if cleanup fails
    let _ = cleanup()
}
```

---

## 14.3 Performance

### 14.3.1 Avoid Unnecessary Allocations

```aether
// Bad: Allocates new string
func greet(name: String) {
    print(f"Hello, {name}")
}

// Good: Borrow instead
func greet(name: &str) {
    print(f"Hello, {name}")
}
```

### 14.3.2 Use Iterators

```aether
// Bad: Allocates intermediate collections
let result = data
    .collect_to_vec()
    .filter(|x| x > 0)
    .collect_to_vec()
    .map(|x| x * 2)
    .collect_to_vec()

// Good: Lazy iteration
let result: Vec<Int> = data
    .iter()
    .filter(|x| *x > 0)
    .map(|x| x * 2)
    .collect()
```

### 14.3.3 Prefer Stack Over Heap

```aether
// Bad: Unnecessary heap allocation
let boxed = Box::new([1, 2, 3])

// Good: Stack allocation when possible
let array = [1, 2, 3]
```

### 14.3.4 Use Capacity Hints

```aether
// Bad: Many reallocations
let mut v = Vec::new()
for i in 0..1000 {
    v.push(i)  // May reallocate multiple times
}

// Good: Pre-allocate
let mut v = Vec::with_capacity(1000)
for i in 0..1000 {
    v.push(i)  // No reallocations
}
```

---

## 14.4 API Design

### 14.4.1 Accept References When Possible

```aether
// Bad: Forces caller to clone
func process(data: Vec<Int>) { ... }

// Good: Accept reference
func process(data: &[Int]) { ... }
```

### 14.4.2 Return Values, Not References

```aether
// Usually bad: Lifetime complexity
func get_name(&self) -> &String {
    &self.name
}

// Usually good: Return owned value
func get_name(&self) -> String {
    self.name.clone()
}

// Or return reference to primitive
func get_age(&self) -> Int {
    self.age  // Copy is cheap
}
```

### 14.4.3 Use Builder Pattern for Complex Construction

```aether
// Bad: Many parameters
func create_config(
    host: String,
    port: Int,
    timeout: Duration,
    retries: Int,
    verbose: Bool,
) -> Config { ... }

// Good: Builder pattern
let config = Config::builder()
    .host("localhost")
    .port(8080)
    .timeout(Duration::seconds(30))
    .retries(3)
    .verbose(true)
    .build()
```

### 14.4.4 Make Invalid States Unrepresentable

```aether
// Bad: Invalid states possible
struct Connection {
    connected: Bool,
    socket: Option<Socket>,  // Can be None when connected=true
}

// Good: Use enum
enum Connection {
    Disconnected,
    Connected(Socket),
}
```

---

## 14.5 Documentation

### 14.5.1 Document Public Items

```aether
/// Creates a new user with the given name.
/// 
/// # Arguments
/// 
/// * `name` - The user's display name (1-100 characters)
/// 
/// # Returns
/// 
/// A new User instance
/// 
/// # Errors
/// 
/// Returns `UserError::InvalidName` if name is empty or too long
/// 
/// # Examples
/// 
/// ```
/// let user = User::new("Alice")?;
/// assert_eq!(user.name(), "Alice");
/// ```
pub func new(name: &str) -> Result<User, UserError> {
    // ...
}
```

### 14.5.2 Use Examples

```aether
/// Parses a JSON string into a Value.
/// 
/// # Examples
/// 
/// ```
/// let json = r#"{"name": "Alice", "age": 30}"#;
/// let value = parse(json)?;
/// 
/// assert_eq!(value["name"], "Alice");
/// assert_eq!(value["age"], 30);
/// ```
pub func parse(input: &str) -> Result<Value, ParseError>
```

### 14.5.3 Document Panics

```aether
/// Gets the element at the given index.
/// 
/// # Panics
/// 
/// Panics if `index >= len()`.
/// 
/// Use `get()` for a non-panicking alternative.
pub func index(self, index: Int) -> &T
```

---

## 14.6 Testing

### 14.6.1 Test Happy Path AND Edge Cases

```aether
#[test]
func test_parse_valid() {
    assert_eq!(parse("42"), Ok(42))
}

#[test]
func test_parse_negative() {
    assert_eq!(parse("-17"), Ok(-17))
}

#[test]
func test_parse_zero() {
    assert_eq!(parse("0"), Ok(0))
}

#[test]
func test_parse_empty() {
    assert!(parse("").is_err())
}

#[test]
func test_parse_invalid() {
    assert!(parse("abc").is_err())
}

#[test]
func test_parse_overflow() {
    assert!(parse("999999999999999999999").is_err())
}
```

### 14.6.2 Use Descriptive Test Names

```aether
// Bad
#[test]
func test1() { ... }

// Good
#[test]
func test_user_creation_with_valid_email_succeeds() { ... }

#[test]
func test_user_creation_with_invalid_email_fails() { ... }
```

### 14.6.3 One Assertion Per Test (Usually)

```aether
// Bad: Multiple unrelated assertions
#[test]
func test_user() {
    let user = User::new("Alice")
    assert_eq!(user.name(), "Alice")
    assert!(user.is_active())
    assert_eq!(user.friends().len(), 0)
}

// Good: Separate tests
#[test]
func test_new_user_has_correct_name() {
    let user = User::new("Alice")
    assert_eq!(user.name(), "Alice")
}

#[test]
func test_new_user_is_active() {
    let user = User::new("Alice")
    assert!(user.is_active())
}

#[test]
func test_new_user_has_no_friends() {
    let user = User::new("Alice")
    assert_eq!(user.friends().len(), 0)
}
```

---

## 14.7 Common Patterns

### 14.7.1 RAII (Resource Acquisition Is Initialization)

```aether
struct File {
    handle: Int,
}

impl File {
    func open(path: &str) -> Result<File, IoError> {
        let handle = sys_open(path)?
        Ok(File { handle })
    }
}

impl Drop for File {
    func drop(mut self) {
        sys_close(self.handle)  // Automatic cleanup
    }
}

// Usage: file automatically closed when out of scope
{
    let file = File::open("data.txt")?
    // Use file...
}  // File closed here
```

### 14.7.2 Newtype Pattern

```aether
// Prevent mixing up different ID types
struct UserId(Int)
struct ProductId(Int)

func get_user(id: UserId) -> User { ... }
func get_product(id: ProductId) -> Product { ... }

// Compile error: can't pass ProductId to get_user
let user = get_user(ProductId(42))  // ERROR!
```

### 14.7.3 Type State Pattern

```aether
// Connection states as types
struct Disconnected
struct Connected
struct Authenticated

struct Connection<State> {
    socket: Socket,
    _state: State,
}

impl Connection<Disconnected> {
    func connect(host: &str) -> Connection<Connected> { ... }
}

impl Connection<Connected> {
    func authenticate(creds: Credentials) -> Connection<Authenticated> { ... }
}

impl Connection<Authenticated> {
    func query(sql: &str) -> Result<Data, Error> { ... }
}

// Can't query without authenticating:
let conn = Connection::connect("db.example.com")
conn.query("SELECT *")  // ERROR: query not available on Connected

let auth_conn = conn.authenticate(creds)
auth_conn.query("SELECT *")  // OK!
```

---

*Next: [Part 15: Complete Example](15_EXAMPLE.md)*
