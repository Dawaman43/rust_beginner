// Shadowing example
//
// Learning goals:
// - Understand shadowing (redeclaring a variable with `let`) vs mutability
// - See that shadowing lets you reuse the same name for a new value

fn main() {
    // First binding of `x` is immutable with value 3
    let x = 3;

    // Shadow `x` by declaring a new `x` that uses the previous value
    let x = x + 3; // new `x` value is 6

    println!("Value of x is {x}"); // prints: Value of x is 6

    // Note: shadowing is different from mutation. The following would be a compile error
    // x = x + 3; // error: cannot assign twice to immutable variable `x`
}