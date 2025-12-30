# Memory Management in Aether

## Memory Model

Aether uses a **manual memory management** model with these regions:

1. **Stack**: Automatic, local variables
2. **Heap**: Manual allocation via `ae_malloc`
3. **Static**: Constants and global data

## Stack Allocation

### Automatic for Local Variables

```aether
func example() -> Int {
    let x = 10      // Stack allocated (8 bytes)
    let y = 20      // Stack allocated (8 bytes)
    let z = 30      // Stack allocated (8 bytes)
    x + y + z       // Stack cleaned up on return
}
```

### Stack Frame Layout

```
High Address
+------------------+
| Previous Frame   |
+------------------+
| Return Address   |
+------------------+
| Frame Pointer    |
+------------------+
| Local: x         | -16 from FP
+------------------+
| Local: y         | -24 from FP
+------------------+
| Local: z         | -32 from FP
+------------------+
Low Address
```

### Stack Size

Default stack: 16KB per function. Deep recursion may need care.

## Heap Allocation

### ae_malloc

Allocate heap memory:

```aether
func allocate_buffer() -> Int {
    let size = 1024
    let ptr = ae_malloc(size)   // Returns address
    ptr
}
```

### Memory Operations

#### Store Operations

```aether
// Store 8-bit (1 byte)
ae_store8(address, value)

// Store 64-bit (8 bytes)
ae_store64(address, value)
```

#### Load Operations

```aether
// Load 8-bit
let byte = ae_load8(address)

// Load 64-bit
let num = ae_load64(address)
```

### Example: Building a Buffer

```aether
func create_buffer() -> Int {
    let buf = ae_malloc(1024)
    
    // Write data
    ae_store64(buf, 100)        // First 8 bytes
    ae_store64(buf + 8, 200)    // Next 8 bytes
    ae_store64(buf + 16, 300)   // Next 8 bytes
    
    buf
}

func read_buffer(buf: Int) -> Int {
    let a = ae_load64(buf)
    let b = ae_load64(buf + 8)
    let c = ae_load64(buf + 16)
    a + b + c   // 600
}
```

## Arrays

### Creating Arrays

```aether
func create_array(size: Int) -> Int {
    let arr = ae_malloc(size * 8)   // 8 bytes per element
    arr
}
```

### Accessing Elements

```aether
func array_get(arr: Int, index: Int) -> Int {
    ae_load64(arr + index * 8)
}

func array_set(arr: Int, index: Int, value: Int) {
    ae_store64(arr + index * 8, value)
}
```

### Array Example

```aether
func sum_array(arr: Int, len: Int) -> Int {
    let sum = 0
    let i = 0
    while i < len {
        sum = sum + ae_load64(arr + i * 8)
        i = i + 1
    }
    sum
}

func main() -> Int {
    let arr = ae_malloc(40)     // 5 elements
    ae_store64(arr, 10)
    ae_store64(arr + 8, 20)
    ae_store64(arr + 16, 30)
    ae_store64(arr + 24, 40)
    ae_store64(arr + 32, 50)
    
    sum_array(arr, 5)   // 150
}
```

## Strings

### Building Strings (Byte Arrays)

```aether
func make_hello() -> Int {
    let s = ae_malloc(6)    // "Hello" + null
    ae_store8(s, 72)        // 'H'
    ae_store8(s + 1, 101)   // 'e'
    ae_store8(s + 2, 108)   // 'l'
    ae_store8(s + 3, 108)   // 'l'
    ae_store8(s + 4, 111)   // 'o'
    ae_store8(s + 5, 0)     // null terminator
    s
}
```

### String Length

```aether
func strlen(s: Int) -> Int {
    let len = 0
    while ae_load8(s + len) != 0 {
        len = len + 1
    }
    len
}
```

### String Comparison

```aether
func strcmp(s1: Int, s2: Int) -> Int {
    let i = 0
    while 1 {
        let c1 = ae_load8(s1 + i)
        let c2 = ae_load8(s2 + i)
        if c1 != c2 {
            return c1 - c2
        }
        if c1 == 0 {
            return 0
        }
        i = i + 1
    }
    0
}
```

## Data Structures

### Struct Pattern (Manual Layout)

```aether
// Point: { x: Int, y: Int }
const POINT_SIZE: Int = 16
const POINT_X: Int = 0
const POINT_Y: Int = 8

func point_new(x: Int, y: Int) -> Int {
    let p = ae_malloc(POINT_SIZE)
    ae_store64(p + POINT_X, x)
    ae_store64(p + POINT_Y, y)
    p
}

func point_x(p: Int) -> Int { ae_load64(p + POINT_X) }
func point_y(p: Int) -> Int { ae_load64(p + POINT_Y) }
```

### Linked List

```aether
// Node: { value: Int, next: Int }
const NODE_SIZE: Int = 16
const NODE_VALUE: Int = 0
const NODE_NEXT: Int = 8

func node_new(value: Int) -> Int {
    let n = ae_malloc(NODE_SIZE)
    ae_store64(n + NODE_VALUE, value)
    ae_store64(n + NODE_NEXT, 0)    // null
    n
}

func list_push(head: Int, value: Int) -> Int {
    let node = node_new(value)
    ae_store64(node + NODE_NEXT, head)
    node    // New head
}
```

## Memory Best Practices

### 1. Always Initialize Memory

```aether
func safe_alloc(size: Int) -> Int {
    let ptr = ae_malloc(size)
    // Zero-initialize
    let i = 0
    while i < size {
        ae_store8(ptr + i, 0)
        i = i + 1
    }
    ptr
}
```

### 2. Check Allocations

```aether
func create_buffer(size: Int) -> Int {
    let ptr = ae_malloc(size)
    if ptr == 0 {
        return 0    // Allocation failed
    }
    ptr
}
```

### 3. Use Consistent Layouts

Define constants for struct offsets:

```aether
const RECT_X: Int = 0
const RECT_Y: Int = 8
const RECT_W: Int = 16
const RECT_H: Int = 24
const RECT_SIZE: Int = 32
```
