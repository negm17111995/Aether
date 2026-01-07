const std = @import("std");

fn fib(n: i64) i64 {
    if (n <= 1) return n;
    return fib(n - 1) + fib(n - 2);
}

fn bench_fib() i64 {
    return fib(42);
}

fn bench_alloc(allocator: std.mem.Allocator) !i64 {
    const count: usize = 10000000;
    var sum: i64 = 0;
    var i: usize = 0;
    while (i < count) : (i += 1) {
        const ptr = try allocator.create(i64);
        ptr.* = @intCast(i);
        sum += ptr.*;
        // No free
    }
    return sum;
}

fn process_request(id: i64) i64 {
    var acc: i64 = 0;
    var i: i64 = 0;
    while (i < 1000) : (i += 1) {
        acc += i * id;
    }
    return acc;
}

fn bench_requests() i64 {
    const total: usize = 1000000;
    var completed: i64 = 0;
    var i: i64 = 0;
    while (i < total) : (i += 1) {
        if (process_request(i) > 0) {
            completed += 1;
        }
    }
    return completed;
}

fn bench_sieve(allocator: std.mem.Allocator) !i64 {
    const limit: usize = 50000000;
    var is_prime = try allocator.alloc(bool, limit);
    @memset(is_prime, true);

    var i: usize = 2;
    while (i * i < limit) : (i += 1) {
        if (is_prime[i]) {
            var j = i * i;
            while (j < limit) : (j += i) {
                is_prime[j] = false;
            }
        }
    }

    var count: i64 = 0;
    i = 2;
    while (i < limit) : (i += 1) {
        if (is_prime[i]) count += 1;
    }
    return count;
}

pub fn main() !void {
    // Arena allocator to fake 'no free' behavior efficiently or GPA to leak
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    // defer arena.deinit(); // Leak intentionally like others
    const allocator = arena.allocator();

    const f = bench_fib();
    const a = try bench_alloc(allocator);
    const r = bench_requests();
    const s = try bench_sieve(allocator);
    std.debug.print("{d}\n", .{f + a + r + s});
}
