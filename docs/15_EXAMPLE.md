# PART 15: COMPLETE EXAMPLE

## Building a REST API Server

This example demonstrates a complete, production-ready REST API with:
- HTTP server
- JSON handling
- Database abstraction
- Authentication
- Error handling
- Testing

---

## 15.1 Project Structure

```
todo-api/
├── Aether.toml          # Package configuration
├── src/
│   ├── main.aether      # Entry point
│   ├── lib.aether       # Library root
│   ├── config.aether    # Configuration
│   ├── errors.aether    # Error types
│   ├── models/
│   │   ├── mod.aether
│   │   ├── user.aether
│   │   └── todo.aether
│   ├── handlers/
│   │   ├── mod.aether
│   │   ├── auth.aether
│   │   └── todos.aether
│   ├── middleware/
│   │   ├── mod.aether
│   │   └── auth.aether
│   └── db/
│       ├── mod.aether
│       └── memory.aether
└── tests/
    ├── auth_test.aether
    └── todos_test.aether
```

---

## 15.2 Configuration

```aether
// src/config.aether

/// Application configuration
pub struct Config {
    pub host: String,
    pub port: Int,
    pub jwt_secret: String,
    pub log_level: LogLevel,
}

pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl Config {
    /// Load configuration from environment
    pub func from_env() -> Result<Config, ConfigError> {
        Ok(Config {
            host: env::get("HOST").unwrap_or("0.0.0.0"),
            port: env::get("PORT")
                .and_then(|p| p.parse())
                .unwrap_or(8080),
            jwt_secret: env::get("JWT_SECRET")
                .ok_or(ConfigError::MissingEnv("JWT_SECRET"))?,
            log_level: LogLevel::Info,
        })
    }
}

pub enum ConfigError {
    MissingEnv(String),
    InvalidValue(String),
}

impl Error for ConfigError {
    func message(self) -> String {
        match self {
            ConfigError::MissingEnv(key) => 
                f"Missing required environment variable: {key}",
            ConfigError::InvalidValue(msg) => 
                f"Invalid configuration value: {msg}",
        }
    }
}
```

---

## 15.3 Error Types

```aether
// src/errors.aether

use std::net::http::StatusCode

/// Application error types
pub enum AppError {
    NotFound(String),
    Unauthorized(String),
    BadRequest(String),
    Internal(String),
    Validation(Vec<String>),
}

impl AppError {
    pub func status_code(self) -> StatusCode {
        match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    
    pub func to_json(self) -> String {
        match self {
            AppError::NotFound(msg) => 
                f#"{{"error": "not_found", "message": "{msg}"}}"#,
            AppError::Unauthorized(msg) => 
                f#"{{"error": "unauthorized", "message": "{msg}"}}"#,
            AppError::BadRequest(msg) => 
                f#"{{"error": "bad_request", "message": "{msg}"}}"#,
            AppError::Validation(errors) => {
                let errs = errors.join(", ")
                f#"{{"error": "validation", "messages": ["{errs}"]}}"#
            },
            AppError::Internal(msg) => 
                f#"{{"error": "internal", "message": "{msg}"}}"#,
        }
    }
}
```

---

## 15.4 Models

```aether
// src/models/user.aether

use std::time::DateTime

#[derive(Clone, Debug, Eq)]
pub struct User {
    pub id: Int,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub created_at: DateTime,
}

impl User {
    pub func new(email: String, password: String, name: String) -> User {
        User {
            id: 0,  // Set by database
            email,
            password_hash: hash_password(password),
            name,
            created_at: DateTime::now(),
        }
    }
    
    pub func verify_password(self, password: String) -> Bool {
        verify_hash(self.password_hash, password)
    }
    
    /// Convert to JSON (excluding password)
    pub func to_json(self) -> String {
        f#"{{
            "id": {self.id},
            "email": "{self.email}",
            "name": "{self.name}",
            "created_at": "{self.created_at}"
        }}"#
    }
}

#[derive(Clone, Debug)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

impl CreateUserRequest {
    pub func validate(self) -> Result<(), Vec<String>> {
        let mut errors = []
        
        if self.email.is_empty() {
            errors.push("email is required")
        }
        
        if !self.email.contains("@") {
            errors.push("email must be valid")
        }
        
        if self.password.len() < 8 {
            errors.push("password must be at least 8 characters")
        }
        
        if self.name.is_empty() {
            errors.push("name is required")
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

```aether
// src/models/todo.aether

use std::time::DateTime

#[derive(Clone, Debug, Eq)]
pub struct Todo {
    pub id: Int,
    pub user_id: Int,
    pub title: String,
    pub description: Option<String>,
    pub completed: Bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Todo {
    pub func new(user_id: Int, title: String, description: Option<String>) -> Todo {
        let now = DateTime::now()
        Todo {
            id: 0,
            user_id,
            title,
            description,
            completed: false,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub func complete(mut self) {
        self.completed = true
        self.updated_at = DateTime::now()
    }
    
    pub func to_json(self) -> String {
        let desc = match self.description {
            Some(d) => f#""{d}""#,
            None => "null",
        }
        
        f#"{{
            "id": {self.id},
            "user_id": {self.user_id},
            "title": "{self.title}",
            "description": {desc},
            "completed": {self.completed},
            "created_at": "{self.created_at}",
            "updated_at": "{self.updated_at}"
        }}"#
    }
}

#[derive(Clone, Debug)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: Option<String>,
}

impl CreateTodoRequest {
    pub func validate(self) -> Result<(), Vec<String>> {
        let mut errors = []
        
        if self.title.is_empty() {
            errors.push("title is required")
        }
        
        if self.title.len() > 200 {
            errors.push("title must be 200 characters or less")
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

---

## 15.5 Database Layer

```aether
// src/db/mod.aether

use crate::models::{User, Todo}
use crate::errors::AppError

/// Database trait for testing and swapping implementations
pub trait Database {
    // Users
    func create_user(mut self, user: User) -> Result<User, AppError>
    func find_user_by_email(self, email: &str) -> Option<User>
    func find_user_by_id(self, id: Int) -> Option<User>
    
    // Todos
    func create_todo(mut self, todo: Todo) -> Result<Todo, AppError>
    func find_todos_by_user(self, user_id: Int) -> Vec<Todo>
    func find_todo_by_id(self, id: Int) -> Option<Todo>
    func update_todo(mut self, todo: Todo) -> Result<Todo, AppError>
    func delete_todo(mut self, id: Int) -> Result<(), AppError>
}
```

```aether
// src/db/memory.aether

use std::sync::{Arc, Mutex}
use std::collections::HashMap

use crate::db::Database
use crate::models::{User, Todo}
use crate::errors::AppError

/// In-memory database for testing
pub struct MemoryDb {
    users: Arc<Mutex<HashMap<Int, User>>>,
    todos: Arc<Mutex<HashMap<Int, Todo>>>,
    next_user_id: Arc<Mutex<Int>>,
    next_todo_id: Arc<Mutex<Int>>,
}

impl MemoryDb {
    pub func new() -> MemoryDb {
        MemoryDb {
            users: Arc::new(Mutex::new(HashMap::new())),
            todos: Arc::new(Mutex::new(HashMap::new())),
            next_user_id: Arc::new(Mutex::new(1)),
            next_todo_id: Arc::new(Mutex::new(1)),
        }
    }
}

impl Database for MemoryDb {
    func create_user(mut self, mut user: User) -> Result<User, AppError> {
        let mut id = self.next_user_id.lock()
        user.id = *id
        *id += 1
        
        let mut users = self.users.lock()
        users.insert(user.id, user.clone())
        
        Ok(user)
    }
    
    func find_user_by_email(self, email: &str) -> Option<User> {
        let users = self.users.lock()
        users.values()
            .find(|u| u.email == email)
            .cloned()
    }
    
    func find_user_by_id(self, id: Int) -> Option<User> {
        let users = self.users.lock()
        users.get(&id).cloned()
    }
    
    func create_todo(mut self, mut todo: Todo) -> Result<Todo, AppError> {
        let mut id = self.next_todo_id.lock()
        todo.id = *id
        *id += 1
        
        let mut todos = self.todos.lock()
        todos.insert(todo.id, todo.clone())
        
        Ok(todo)
    }
    
    func find_todos_by_user(self, user_id: Int) -> Vec<Todo> {
        let todos = self.todos.lock()
        todos.values()
            .filter(|t| t.user_id == user_id)
            .cloned()
            .collect()
    }
    
    func find_todo_by_id(self, id: Int) -> Option<Todo> {
        let todos = self.todos.lock()
        todos.get(&id).cloned()
    }
    
    func update_todo(mut self, todo: Todo) -> Result<Todo, AppError> {
        let mut todos = self.todos.lock()
        
        if !todos.contains_key(&todo.id) {
            return Err(AppError::NotFound(f"Todo {todo.id} not found"))
        }
        
        todos.insert(todo.id, todo.clone())
        Ok(todo)
    }
    
    func delete_todo(mut self, id: Int) -> Result<(), AppError> {
        let mut todos = self.todos.lock()
        
        if todos.remove(&id).is_none() {
            return Err(AppError::NotFound(f"Todo {id} not found"))
        }
        
        Ok(())
    }
}
```

---

## 15.6 Handlers

```aether
// src/handlers/auth.aether

use std::net::http::{Request, Response}
use crate::db::Database
use crate::models::{User, CreateUserRequest}
use crate::errors::AppError

pub struct AuthHandler<D: Database> {
    db: D,
    jwt_secret: String,
}

impl<D: Database> AuthHandler<D> {
    pub func new(db: D, jwt_secret: String) -> AuthHandler<D> {
        AuthHandler { db, jwt_secret }
    }
    
    /// POST /register
    pub func register(mut self, req: Request) -> Response {
        // Parse request body
        let body: CreateUserRequest = match parse_json(req.body) {
            Ok(b) => b,
            Err(e) => return error_response(AppError::BadRequest(e.message())),
        }
        
        // Validate
        match body.validate() {
            Ok(()) => {},
            Err(errors) => return error_response(AppError::Validation(errors)),
        }
        
        // Check if email exists
        if self.db.find_user_by_email(&body.email).is_some() {
            return error_response(AppError::BadRequest("Email already exists"))
        }
        
        // Create user
        let user = User::new(body.email, body.password, body.name)
        let user = match self.db.create_user(user) {
            Ok(u) => u,
            Err(e) => return error_response(e),
        }
        
        // Generate token
        let token = generate_jwt(user.id, self.jwt_secret)
        
        Response::json(f#"{{
            "user": {user.to_json()},
            "token": "{token}"
        }}"#)
    }
    
    /// POST /login
    pub func login(self, req: Request) -> Response {
        let body: LoginRequest = match parse_json(req.body) {
            Ok(b) => b,
            Err(e) => return error_response(AppError::BadRequest(e.message())),
        }
        
        // Find user
        let user = match self.db.find_user_by_email(&body.email) {
            Some(u) => u,
            None => return error_response(AppError::Unauthorized("Invalid credentials")),
        }
        
        // Verify password
        if !user.verify_password(body.password) {
            return error_response(AppError::Unauthorized("Invalid credentials"))
        }
        
        // Generate token
        let token = generate_jwt(user.id, self.jwt_secret)
        
        Response::json(f#"{{
            "user": {user.to_json()},
            "token": "{token}"
        }}"#)
    }
}

struct LoginRequest {
    email: String,
    password: String,
}
```

```aether
// src/handlers/todos.aether

use std::net::http::{Request, Response}
use crate::db::Database
use crate::models::{Todo, CreateTodoRequest}
use crate::errors::AppError

pub struct TodoHandler<D: Database> {
    db: D,
}

impl<D: Database> TodoHandler<D> {
    pub func new(db: D) -> TodoHandler<D> {
        TodoHandler { db }
    }
    
    /// GET /todos
    pub func list(self, user_id: Int) -> Response {
        let todos = self.db.find_todos_by_user(user_id)
        let json = todos.iter()
            .map(|t| t.to_json())
            .collect::<Vec<_>>()
            .join(",")
        
        Response::json(f"[{json}]")
    }
    
    /// POST /todos
    pub func create(mut self, user_id: Int, req: Request) -> Response {
        let body: CreateTodoRequest = match parse_json(req.body) {
            Ok(b) => b,
            Err(e) => return error_response(AppError::BadRequest(e.message())),
        }
        
        match body.validate() {
            Ok(()) => {},
            Err(errors) => return error_response(AppError::Validation(errors)),
        }
        
        let todo = Todo::new(user_id, body.title, body.description)
        let todo = match self.db.create_todo(todo) {
            Ok(t) => t,
            Err(e) => return error_response(e),
        }
        
        Response::json(todo.to_json())
            .status(201)
    }
    
    /// GET /todos/:id
    pub func get(self, user_id: Int, todo_id: Int) -> Response {
        let todo = match self.db.find_todo_by_id(todo_id) {
            Some(t) => t,
            None => return error_response(AppError::NotFound("Todo not found")),
        }
        
        if todo.user_id != user_id {
            return error_response(AppError::NotFound("Todo not found"))
        }
        
        Response::json(todo.to_json())
    }
    
    /// PUT /todos/:id/complete
    pub func complete(mut self, user_id: Int, todo_id: Int) -> Response {
        let mut todo = match self.db.find_todo_by_id(todo_id) {
            Some(t) => t,
            None => return error_response(AppError::NotFound("Todo not found")),
        }
        
        if todo.user_id != user_id {
            return error_response(AppError::NotFound("Todo not found"))
        }
        
        todo.complete()
        
        let todo = match self.db.update_todo(todo) {
            Ok(t) => t,
            Err(e) => return error_response(e),
        }
        
        Response::json(todo.to_json())
    }
    
    /// DELETE /todos/:id
    pub func delete(mut self, user_id: Int, todo_id: Int) -> Response {
        let todo = match self.db.find_todo_by_id(todo_id) {
            Some(t) => t,
            None => return error_response(AppError::NotFound("Todo not found")),
        }
        
        if todo.user_id != user_id {
            return error_response(AppError::NotFound("Todo not found"))
        }
        
        match self.db.delete_todo(todo_id) {
            Ok(()) => Response::no_content(),
            Err(e) => error_response(e),
        }
    }
}
```

---

## 15.7 Main Entry Point

```aether
// src/main.aether

use std::net::http

use crate::config::Config
use crate::db::MemoryDb
use crate::handlers::{AuthHandler, TodoHandler}
use crate::middleware::AuthMiddleware

func main() -> Int {
    // Load configuration
    let config = match Config::from_env() {
        Ok(c) => c,
        Err(e) => {
            eprintln(f"Configuration error: {e.message()}")
            return 1
        }
    }
    
    // Initialize database
    let db = MemoryDb::new()
    
    // Create handlers
    let auth_handler = AuthHandler::new(db.clone(), config.jwt_secret.clone())
    let todo_handler = TodoHandler::new(db.clone())
    let auth_middleware = AuthMiddleware::new(config.jwt_secret.clone())
    
    println(f"Starting server on {config.host}:{config.port}")
    
    // Start server
    http::serve(config.port, |req| {
        // Route matching
        match (req.method, req.path.as_str()) {
            // Public routes
            ("POST", "/register") => auth_handler.register(req),
            ("POST", "/login") => auth_handler.login(req),
            
            // Protected routes
            ("GET", "/todos") => {
                match auth_middleware.authenticate(&req) {
                    Ok(user_id) => todo_handler.list(user_id),
                    Err(e) => error_response(e),
                }
            },
            ("POST", "/todos") => {
                match auth_middleware.authenticate(&req) {
                    Ok(user_id) => todo_handler.create(user_id, req),
                    Err(e) => error_response(e),
                }
            },
            
            // 404 for unknown routes
            _ => http::Response::not_found(),
        }
    })
    
    0
}

func error_response(err: AppError) -> http::Response {
    http::Response::json(err.to_json())
        .status(err.status_code())
}
```

---

## 15.8 Tests

```aether
// tests/todos_test.aether

use crate::db::MemoryDb
use crate::handlers::TodoHandler
use crate::models::{User, Todo}

#[test]
func test_create_todo() {
    let mut db = MemoryDb::new()
    
    // Create user first
    let user = User::new("test@example.com", "password", "Test")
    let user = db.create_user(user).unwrap()
    
    // Create handler
    let mut handler = TodoHandler::new(db)
    
    // Simulate request
    let req = mock_request(r#"{"title": "Test Todo"}"#)
    let response = handler.create(user.id, req)
    
    assert_eq!(response.status, 201)
    assert!(response.body.contains("Test Todo"))
}

#[test]
func test_list_todos_only_shows_user_todos() {
    let mut db = MemoryDb::new()
    
    // Create two users
    let user1 = db.create_user(User::new("u1@test.com", "pass", "U1")).unwrap()
    let user2 = db.create_user(User::new("u2@test.com", "pass", "U2")).unwrap()
    
    // Create todos for each
    db.create_todo(Todo::new(user1.id, "User1 Todo", None)).unwrap()
    db.create_todo(Todo::new(user2.id, "User2 Todo", None)).unwrap()
    
    let handler = TodoHandler::new(db)
    
    // User 1 should only see their todo
    let response = handler.list(user1.id)
    assert!(response.body.contains("User1 Todo"))
    assert!(!response.body.contains("User2 Todo"))
}

#[test]
func test_complete_todo() {
    let mut db = MemoryDb::new()
    let user = db.create_user(User::new("test@test.com", "pass", "T")).unwrap()
    let todo = db.create_todo(Todo::new(user.id, "Todo", None)).unwrap()
    
    let mut handler = TodoHandler::new(db)
    let response = handler.complete(user.id, todo.id)
    
    assert_eq!(response.status, 200)
    assert!(response.body.contains(r#""completed": true"#))
}

#[test]
func test_cannot_access_other_users_todo() {
    let mut db = MemoryDb::new()
    let user1 = db.create_user(User::new("u1@test.com", "pass", "U1")).unwrap()
    let user2 = db.create_user(User::new("u2@test.com", "pass", "U2")).unwrap()
    
    let todo = db.create_todo(Todo::new(user1.id, "Private", None)).unwrap()
    
    let handler = TodoHandler::new(db)
    let response = handler.get(user2.id, todo.id)  // User2 tries to access User1's todo
    
    assert_eq!(response.status, 404)
}
```

---

## Summary

This example demonstrated:

1. **Project Organization** - Clean separation of concerns
2. **Configuration** - Environment-based config with validation
3. **Error Handling** - Typed errors with proper HTTP responses
4. **Models** - Data structures with validation
5. **Database Abstraction** - Trait-based for testability
6. **Handlers** - HTTP request processing
7. **Authentication** - JWT-based auth middleware
8. **Testing** - Unit tests with mock database

---

*Congratulations! You are now an Aether professional!* 🎉
