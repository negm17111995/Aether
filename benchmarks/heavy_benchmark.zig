// Zig HEAVY BENCHMARK
const std = @import("std");

// TEST 1: FIBONACCI
fn fib(n: i64) i64 {
    if (n <= 1) return n;
    return fib(n - 1) + fib(n - 2);
}

fn benchFib() i64 {
    const start = std.time.milliTimestamp();
    const result = fib(40);
    const end = std.time.milliTimestamp();
    const elapsed = end - start;
    std.debug.print("Fibonacci(40) = {} in {}ms\n", .{result, elapsed});
    return elapsed;
}

// TEST 2: MEMORY ALLOCATION
fn benchAlloc(allocator: std.mem.Allocator, count: usize) !i64 {
    const start = std.time.milliTimestamp();
    var blocks = try allocator.alloc([]u8, count);
    defer allocator.free(blocks);
    
    for (0..count) |i| {
        blocks[i] = try allocator.alloc(u8, 1024);
        blocks[i][0] = @intCast(i % 256);
    }
    
    const end = std.time.milliTimestamp();
    const elapsed = end - start;
    std.debug.print("Allocated {} x 1KB blocks in {}ms\n", .{count, elapsed});
    return elapsed;
}

// TEST 3: CONCURRENT SIMULATION  
fn processRequest(id: usize) i64 {
    _ = id;
    var sum: i64 = 0;
    for (0..10000) |i| {
        sum += @intCast(i);
    }
    return sum;
}

fn benchConcurrent(requests: usize) i64 {
    const start = std.time.milliTimestamp();
    var completed: usize = 0;
    
    for (0..requests) |i| {
        if (processRequest(i) > 0) completed += 1;
    }
    
    const end = std.time.milliTimestamp();
    const elapsed = end - start;
    std.debug.print("Processed {} requests in {}ms\n", .{completed, elapsed});
    return elapsed;
}

// TEST 4: PRIME SIEVE
fn sieve(allocator: std.mem.Allocator, limit: usize) !usize {
    var is_prime = try allocator.alloc(bool, limit);
    defer allocator.free(is_prime);
    
    @memset(is_prime, true);
    is_prime[0] = false;
    is_prime[1] = false;
    
    var i: usize = 2;
    while (i * i < limit) : (i += 1) {
        if (is_prime[i]) {
            var j = i * i;
            while (j < limit) : (j += i) {
                is_prime[j] = false;
            }
        }
    }
    
    var count: usize = 0;
    for (is_prime) |p| {
        if (p) count += 1;
    }
    return count;
}

fn benchSieve(allocator: std.mem.Allocator) !i64 {
    const start = std.time.milliTimestamp();
    const primes = try sieve(allocator, 10_000_000);
    const end = std.time.milliTimestamp();
    const elapsed = end - start;
    std.debug.print("Found {} primes under 10M in {}ms\n", .{primes, elapsed});
    return elapsed;
}

// TEST 5: VECTOR OPERATIONS
fn benchVector(allocator: std.mem.Allocator, size: usize) !i64 {
    const start = std.time.milliTimestamp();
    
    var a = try allocator.alloc(i64, size);
    var b = try allocator.alloc(i64, size);
    var c = try allocator.alloc(i64, size);
    defer allocator.free(a);
    defer allocator.free(b);
    defer allocator.free(c);
    
    for (0..size) |i| {
        a[i] = @intCast(i);
        b[i] = @intCast(i * 2);
    }
    
    for (0..size) |i| {
        c[i] = a[i] + b[i];
    }
    
    const end = std.time.milliTimestamp();
    const elapsed = end - start;
    std.debug.print("Vector add of {} elements in {}ms\n", .{size, elapsed});
    return elapsed;
}

pub fn main() !void {
    std.debug.print("=== ZIG HEAVY BENCHMARK ===\n\n", .{});
    
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    
    var total: i64 = 0;
    
    std.debug.print("TEST 1: Fibonacci (Recursive)\n", .{});
    total += benchFib();
    
    std.debug.print("\nTEST 2: Memory Allocation (1M blocks)\n", .{});
    total += try benchAlloc(allocator, 1_000_000);
    
    std.debug.print("\nTEST 3: Concurrent Requests (100K)\n", .{});
    total += benchConcurrent(100_000);
    
    std.debug.print("\nTEST 4: Prime Sieve (10M)\n", .{});
    total += try benchSieve(allocator);
    
    std.debug.print("\nTEST 5: Vector Operations (10M)\n", .{});
    total += try benchVector(allocator, 10_000_000);
    
    std.debug.print("\n=== TOTAL: {}ms ===\n", .{total});
}
