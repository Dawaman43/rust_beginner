// Tuples in Rust
//
// Learning goals:
// - Group different types into one compound type using tuples
// - Destructure tuples and access fields by index

fn main() {
    // A tuple can hold values of different types
    let tup = (1, 1.4, "string");

    // Destructure the tuple into separate variables
    let (x, y, z) = tup;
    println!("The value of x: {x}"); // prints: 1
    println!("The value of y: {y}"); // prints: 1.4
    println!("The value of z: {z}"); // prints: string

    // You can also access tuple elements by index (zero-based)
    println!("The second element in the tuple is: {}", tup.1);
}