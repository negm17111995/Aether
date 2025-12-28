# PART 9: CONCURRENCY

## 9.1 Introduction to Concurrency

### 9.1.1 What is Concurrency?

Concurrency = multiple tasks making progress simultaneously.

Aether provides:
- **Spawning threads** - Run code in parallel
- **Channels** - Safe message passing
- **Atomics** - Lock-free synchronization
- **Async/Await** - Non-blocking I/O

### 9.1.2 Aether's Concurrency Guarantees

The type system prevents:
- Data races (multiple threads writing same data)
- Use-after-free in concurrent code
- Deadlocks (detected at compile time when possible)

---

## 9.2 Spawning Threads

### 9.2.1 Basic Spawn

```aether
use std::thread

func main() {
    // Spawn a new thread
    let handle = spawn(|| {
        print("Hello from thread!")
    })
    
    // Wait for thread to finish
    handle.join()
}
```

### 9.2.2 Multiple Threads

```aether
let mut handles = []

for i in 0..10 {
    let h = spawn(|| {
        print(f"Thread {i}")
    })
    handles.push(h)
}

// Wait for all
for h in handles {
    h.join()
}
```

### 9.2.3 Thread with Return Value

```aether
let handle = spawn(|| {
    42  // Return value
})

let result = handle.join()  // Result<Int, Error>
print(result.unwrap())  // 42
```

### 9.2.4 Capturing Variables

```aether
let data = [1, 2, 3, 4, 5]

// Move data into thread
let handle = spawn(move || {
    let sum = 0
    for x in data {
        sum = sum + x
    }
    sum
})

let total = handle.join().unwrap()  // 15
```

---

## 9.3 Channels

### 9.3.1 Creating Channels

```aether
use std::sync::channel

// Create channel
let (sender, receiver) = channel()

// Send
sender.send(42)

// Receive
let value = receiver.recv()  // 42
```

### 9.3.2 Multiple Senders

```aether
let (tx, rx) = channel()

for i in 0..5 {
    let sender = tx.clone()  // Clone sender for each thread
    spawn(move || {
        sender.send(i)
    })
}

// Receive all messages
for _ in 0..5 {
    print(rx.recv())
}
```

### 9.3.3 Buffered Channels

```aether
// Unbuffered: blocks until received
let (tx, rx) = channel()

// Buffered: can store N messages
let (tx, rx) = channel_with_capacity(10)

// Non-blocking send
match tx.try_send(42) {
    Ok(()) => print("sent"),
    Err(e) => print("buffer full"),
}

// Non-blocking receive
match rx.try_recv() {
    Some(val) => print(val),
    None => print("no message"),
}
```

### 9.3.4 Channel Patterns

**Pipeline:**
```aether
let (tx1, rx1) = channel()
let (tx2, rx2) = channel()

// Stage 1: Generate
spawn(move || {
    for i in 0..100 {
        tx1.send(i)
    }
})

// Stage 2: Process
spawn(move || {
    while let Some(x) = rx1.recv() {
        tx2.send(x * 2)
    }
})

// Stage 3: Consume
while let Some(result) = rx2.recv() {
    print(result)
}
```

---

## 9.4 Shared State

### 9.4.1 Mutex (Mutual Exclusion)

```aether
use std::sync::Mutex

let data = Mutex::new(0)

let mut handles = []

for _ in 0..10 {
    let data_ref = &data
    handles.push(spawn(move || {
        let mut guard = data_ref.lock()
        *guard = *guard + 1
    }))
}

for h in handles {
    h.join()
}

print(*data.lock())  // 10
```

### 9.4.2 RwLock (Read-Write Lock)

```aether
use std::sync::RwLock

let data = RwLock::new([1, 2, 3])

// Multiple readers OK
spawn(|| {
    let guard = data.read()
    print(guard[0])
})

spawn(|| {
    let guard = data.read()
    print(guard[1])
})

// Writer gets exclusive access
spawn(|| {
    let mut guard = data.write()
    guard[0] = 100
})
```

### 9.4.3 Arc + Mutex Pattern

```aether
use std::sync::{Arc, Mutex}

let counter = Arc::new(Mutex::new(0))

let mut handles = []

for _ in 0..100 {
    let c = Arc::clone(&counter)
    handles.push(spawn(move || {
        let mut guard = c.lock()
        *guard = *guard + 1
    }))
}

for h in handles {
    h.join()
}

print(*counter.lock())  // 100
```

---

## 9.5 Atomic Operations

### 9.5.1 Atomic Types

```aether
use std::sync::atomic::{AtomicInt, AtomicBool}

let counter = AtomicInt::new(0)
let flag = AtomicBool::new(false)
```

### 9.5.2 Atomic Operations

```aether
// Load
let value = counter.load()

// Store
counter.store(42)

// Add and get old value
let old = counter.fetch_add(1)

// Compare and swap
let success = counter.compare_exchange(42, 100)
if success {
    print("Swapped 42 -> 100")
}
```

### 9.5.3 Lock-Free Counter

```aether
let counter = AtomicInt::new(0)

let mut handles = []

for _ in 0..1000 {
    let c = &counter
    handles.push(spawn(move || {
        c.fetch_add(1)  // Atomic increment
    }))
}

for h in handles {
    h.join()
}

print(counter.load())  // 1000
```

---

## 9.6 Async/Await

### 9.6.1 Async Functions

```aether
async func fetch_data(url: String) -> String {
    let response = await http_get(url)
    response.body
}
```

### 9.6.2 Awaiting Results

```aether
async func main() {
    let data = await fetch_data("https://api.example.com")
    print(data)
}
```

### 9.6.3 Concurrent Async

```aether
async func fetch_all() {
    // Start all fetches concurrently
    let f1 = async_spawn(fetch_data, "https://api1.com")
    let f2 = async_spawn(fetch_data, "https://api2.com")
    let f3 = async_spawn(fetch_data, "https://api3.com")
    
    // Wait for all
    let results = await_all([f1, f2, f3])
    
    for result in results {
        print(result)
    }
}
```

### 9.6.4 Await with Timeout

```aether
async func fetch_with_timeout() {
    let future = async_spawn(slow_operation)
    
    match await_timeout(future, 5000) {
        Ok(result) => print(result),
        Err(Timeout) => print("Operation timed out"),
    }
}
```

### 9.6.5 Select/Race

```aether
async func race_example() {
    let f1 = async_spawn(slow_fetch)
    let f2 = async_spawn(fast_fetch)
    
    // Get first to complete
    let (result, index) = await_race([f1, f2])
    
    print(f"Task {index} finished first: {result}")
}
```

---

## 9.7 Parallel Iterators

### 9.7.1 Parallel Map

```aether
use std::par

let numbers = [1, 2, 3, 4, 5, 6, 7, 8]

// Process in parallel
let squared = numbers.par_map(|x| x * x)
// [1, 4, 9, 16, 25, 36, 49, 64]
```

### 9.7.2 Parallel Reduce

```aether
let numbers = 1..1000000

let sum = numbers.par_reduce(0, |acc, x| acc + x)
```

### 9.7.3 Parallel Filter

```aether
let numbers = 1..1000

let primes = numbers.par_filter(is_prime)
```

---

## 9.8 Thread Safety

### 9.8.1 Send and Sync Traits

```aether
// Send: can be transferred to another thread
trait Send {}

// Sync: can be shared between threads
trait Sync {}
```

Most types are automatically Send + Sync.

**Not Send:**
- Raw pointers
- Rc<T>

**Not Sync:**
- RefCell<T>
- Cell<T>

### 9.8.2 Thread-Safe Types

| Type | Send | Sync | Use Case |
|------|------|------|----------|
| `Arc<T>` | ✅ | ✅ | Shared ownership across threads |
| `Mutex<T>` | ✅ | ✅ | Exclusive access to data |
| `RwLock<T>` | ✅ | ✅ | Multiple readers, one writer |
| `AtomicInt` | ✅ | ✅ | Lock-free counter |
| `Rc<T>` | ❌ | ❌ | Single-thread only |
| `RefCell<T>` | ✅ | ❌ | Single-thread mutable borrow |

---

*Next: [Part 10: Error Handling](10_ERROR_HANDLING.md)*
