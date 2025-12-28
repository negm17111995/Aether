// Rust Heavy Benchmark - Tak Recursion
fn tak(x: i32, y: i32, z: i32) -> i32 {
    if y >= x {
        z
    } else {
        tak(
            tak(x - 1, y, z),
            tak(y - 1, z, x),
            tak(z - 1, x, y)
        )
    }
}

fn main() {
    let result = tak(30, 20, 10);
    println!("Result: {}", result);
    std::process::exit(result);
}
