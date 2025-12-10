// Arrays in Rust
//
//
// Learning goals:
// - Understand that arrays hold values of the same type and fixed length
// - Learn how to index into an array

fn main() {
    // A fixed-size array of four `i32` values
    let arr = [1, 2, 3, 4];

    // Access elements by index (starting at 0). Indexing will panic at runtime if out of bounds.
    println!("The first element in the array is {}", arr[0]);

    // You can also get the length of the array
    println!("The array has {} elements", arr.len());
}