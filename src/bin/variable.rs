// Variables and mutability in Rust
//
// Learning goals:
// - Understand `let` bindings and immutability
// - Learn how to use `mut` to allow reassignment
// - See compiler behavior when a variable doesn't need to be mutable

fn main() {
    // By default `let` bindings are immutable: you cannot reassign them.
    let x: i32 = 3;
    println!("The value of x before reassigning is {x}");

    // If you uncommenting the following reassignment you'll get a compile error:
    // x = 2; // error: cannot assign twice to immutable variable `x`

    // To allow reassignment, declare the variable as `mut`.
    let mut y: i32 = 4;
    println!("The value of y before reassigning is {y}");
    // Reassigning is allowed because `y` is mutable.
    y = 7;
    println!("The value of y after reassigning is {y}");

    // Note: the compiler warns if you make a variable `mut` but never actually mutate it.
    let z = 9; // immutable, and that's fine when there's no reassignment
    println!("The value of z is {z}");
}