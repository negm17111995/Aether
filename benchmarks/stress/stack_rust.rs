// STRESS TEST - Stack Depth (Rust)
fn recurse(depth: u64) {
    if depth % 100000 == 0 {
        println!("Depth: {}", depth);
    }
    recurse(depth + 1);
}

fn main() {
    println!("Rust Stack Depth Stress Test");
    recurse(0);
}
