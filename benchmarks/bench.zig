// Zig Heavy Benchmark - CPU, Memory, Concurrency
const std = @import("std");

fn isPrime(n: i64) bool {
    if (n < 2) return false;
    if (n == 2) return true;
    if (@mod(n, 2) == 0) return false;
    var i: i64 = 3;
    while (i * i <= n) : (i += 2) {
        if (@mod(n, i) == 0) return false;
    }
    return true;
}

fn countPrimes(limit: i64) i64 {
    var count: i64 = 0;
    var i: i64 = 2;
    while (i < limit) : (i += 1) {
        if (isPrime(i)) count += 1;
    }
    return count;
}

fn memoryStress(allocator: std.mem.Allocator, allocs: i32) i64 {
    var total: i64 = 0;
    var i: i32 = 0;
    while (i < allocs) : (i += 1) {
        const data = allocator.alloc(u8, 1024) catch return total;
        defer allocator.free(data);
        total += i;
    }
    return total;
}

fn fib(n: i64) i64 {
    if (n <= 1) return n;
    var a: i64 = 0;
    var b: i64 = 1;
    var i: i64 = 2;
    while (i <= n) : (i += 1) {
        const c = a + b;
        a = b;
        b = c;
    }
    return b;
}

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    
    const start = std.time.milliTimestamp();
    
    // CPU Test
    const primes = countPrimes(100000);
    
    // Memory Test  
    const mem = memoryStress(allocator, 10000);
    
    // Compute Test
    const f = fib(40);
    
    const end = std.time.milliTimestamp();
    const elapsed = end - start;
    
    try stdout.print("Zig Results:\n", .{});
    try stdout.print("  Primes: {}\n", .{primes});
    try stdout.print("  Fib(40): {}\n", .{f});
    try stdout.print("  Mem: {}\n", .{mem});
    try stdout.print("  Time: {} ms\n", .{elapsed});
}
