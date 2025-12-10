// Constants in Rust
//
// Learning goals:
// - Understand the difference between `const` and `let`
// - Know that `const` values are always immutable and must have a type

fn main() {
    // Constants must have a type and are set at compile time.
    const MAX_POINTS: u32 = 3;
    println!("Constant value MAX_POINTS = {MAX_POINTS}");
}