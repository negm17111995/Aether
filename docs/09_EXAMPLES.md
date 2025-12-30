# Aether Examples

## Basic Examples

### 1. Hello World

```aether
// The simplest Aether program
func main() -> Int {
    0  // Return success
}
```

### 2. Arithmetic

```aether
func main() -> Int {
    let a = 10
    let b = 20
    let sum = a + b
    let product = a * b
    let quotient = product / sum
    quotient  // 200 / 30 = 6
}
```

### 3. Conditional

```aether
func max(a: Int, b: Int) -> Int {
    if a > b {
        return a
    }
    b
}

func main() -> Int {
    max(42, 17)  // 42
}
```

### 4. Loop

```aether
func sum_to(n: Int) -> Int {
    let sum = 0
    let i = 1
    while i <= n {
        sum = sum + i
        i = i + 1
    }
    sum
}

func main() -> Int {
    sum_to(10)  // 55
}
```

## Intermediate Examples

### 5. Factorial

```aether
func factorial(n: Int) -> Int {
    if n <= 1 {
        return 1
    }
    n * factorial(n - 1)
}

func main() -> Int {
    factorial(5)  // 120
}
```

### 6. Fibonacci

```aether
func fib(n: Int) -> Int {
    if n <= 1 {
        return n
    }
    fib(n - 1) + fib(n - 2)
}

func main() -> Int {
    fib(10)  // 55
}
```

### 7. Greatest Common Divisor

```aether
func gcd(a: Int, b: Int) -> Int {
    while b != 0 {
        let t = b
        b = a % b
        a = t
    }
    a
}

func main() -> Int {
    gcd(48, 18)  // 6
}
```

### 8. Prime Check

```aether
func is_prime(n: Int) -> Int {
    if n < 2 { return 0 }
    if n == 2 { return 1 }
    if n % 2 == 0 { return 0 }
    
    let i = 3
    while i * i <= n {
        if n % i == 0 {
            return 0
        }
        i = i + 2
    }
    1
}

func main() -> Int {
    is_prime(97)  // 1 (true)
}
```

### 9. Power Function

```aether
func power(base: Int, exp: Int) -> Int {
    if exp == 0 {
        return 1
    }
    let result = 1
    let i = 0
    while i < exp {
        result = result * base
        i = i + 1
    }
    result
}

func main() -> Int {
    power(2, 10)  // 1024
}
```

### 10. Binary Search

```aether
func binary_search(arr: Int, len: Int, target: Int) -> Int {
    let left = 0
    let right = len - 1
    
    while left <= right {
        let mid = left + (right - left) / 2
        let val = ae_load64(arr + mid * 8)
        
        if val == target {
            return mid
        }
        if val < target {
            left = mid + 1
        } else {
            right = mid - 1
        }
    }
    0 - 1  // Not found
}
```

## Advanced Examples

### 11. Quick Sort

```aether
func swap(arr: Int, i: Int, j: Int) {
    let temp = ae_load64(arr + i * 8)
    ae_store64(arr + i * 8, ae_load64(arr + j * 8))
    ae_store64(arr + j * 8, temp)
}

func partition(arr: Int, low: Int, high: Int) -> Int {
    let pivot = ae_load64(arr + high * 8)
    let i = low - 1
    let j = low
    
    while j < high {
        if ae_load64(arr + j * 8) < pivot {
            i = i + 1
            swap(arr, i, j)
        }
        j = j + 1
    }
    
    swap(arr, i + 1, high)
    i + 1
}

func quicksort(arr: Int, low: Int, high: Int) {
    if low < high {
        let pi = partition(arr, low, high)
        quicksort(arr, low, pi - 1)
        quicksort(arr, pi + 1, high)
    }
}
```

### 12. Linked List

```aether
const NODE_VALUE: Int = 0
const NODE_NEXT: Int = 8

func node_new(value: Int) -> Int {
    let n = ae_malloc(16)
    ae_store64(n + NODE_VALUE, value)
    ae_store64(n + NODE_NEXT, 0)
    n
}

func list_push(head: Int, value: Int) -> Int {
    let node = node_new(value)
    ae_store64(node + NODE_NEXT, head)
    node
}

func list_sum(head: Int) -> Int {
    let sum = 0
    let curr = head
    while curr != 0 {
        sum = sum + ae_load64(curr + NODE_VALUE)
        curr = ae_load64(curr + NODE_NEXT)
    }
    sum
}

func main() -> Int {
    let list = 0
    list = list_push(list, 10)
    list = list_push(list, 20)
    list = list_push(list, 30)
    list_sum(list)  // 60
}
```

### 13. Hash Map

```aether
func hash(key: Int) -> Int {
    let h = 14695981039346656037
    h = h * 1099511628211
    h = h + key
    h
}

func map_new(capacity: Int) -> Int {
    let m = ae_malloc(16 + capacity * 16)
    ae_store64(m, capacity)
    ae_store64(m + 8, 0)
    m
}

func map_put(m: Int, key: Int, value: Int) {
    let cap = ae_load64(m)
    let idx = hash(key) % cap
    let slot = m + 16 + idx * 16
    ae_store64(slot, key)
    ae_store64(slot + 8, value)
}

func map_get(m: Int, key: Int) -> Int {
    let cap = ae_load64(m)
    let idx = hash(key) % cap
    ae_load64(m + 16 + idx * 16 + 8)
}
```

### 14. Matrix Multiplication

```aether
func matrix_mult(a: Int, b: Int, c: Int, n: Int) {
    let i = 0
    while i < n {
        let j = 0
        while j < n {
            let sum = 0
            let k = 0
            while k < n {
                let a_ik = ae_load64(a + (i * n + k) * 8)
                let b_kj = ae_load64(b + (k * n + j) * 8)
                sum = sum + a_ik * b_kj
                k = k + 1
            }
            ae_store64(c + (i * n + j) * 8, sum)
            j = j + 1
        }
        i = i + 1
    }
}
```

### 15. FizzBuzz

```aether
func fizzbuzz_count(n: Int) -> Int {
    let count = 0
    let i = 1
    while i <= n {
        if i % 15 == 0 {
            count = count + 3  // FizzBuzz
        } else {
            if i % 3 == 0 {
                count = count + 1  // Fizz
            } else {
                if i % 5 == 0 {
                    count = count + 2  // Buzz
                }
            }
        }
        i = i + 1
    }
    count
}

func main() -> Int {
    fizzbuzz_count(100)
}
```

## Algorithm Patterns

### Pattern: Two Pointers

```aether
func reverse_array(arr: Int, len: Int) {
    let left = 0
    let right = len - 1
    while left < right {
        let temp = ae_load64(arr + left * 8)
        ae_store64(arr + left * 8, ae_load64(arr + right * 8))
        ae_store64(arr + right * 8, temp)
        left = left + 1
        right = right - 1
    }
}
```

### Pattern: Sliding Window

```aether
func max_sum_subarray(arr: Int, len: Int, k: Int) -> Int {
    let sum = 0
    let i = 0
    while i < k {
        sum = sum + ae_load64(arr + i * 8)
        i = i + 1
    }
    
    let max_sum = sum
    i = k
    while i < len {
        sum = sum + ae_load64(arr + i * 8) - ae_load64(arr + (i - k) * 8)
        if sum > max_sum {
            max_sum = sum
        }
        i = i + 1
    }
    max_sum
}
```

### Pattern: Recursion with Memoization

```aether
func fib_memo(n: Int, memo: Int) -> Int {
    if n <= 1 { return n }
    
    let cached = ae_load64(memo + n * 8)
    if cached != 0 { return cached }
    
    let result = fib_memo(n - 1, memo) + fib_memo(n - 2, memo)
    ae_store64(memo + n * 8, result)
    result
}

func fib_fast(n: Int) -> Int {
    let memo = ae_malloc((n + 1) * 8)
    fib_memo(n, memo)
}
```
