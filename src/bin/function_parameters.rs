// Function parameters example
//
// Learning goals:
// - Define functions that accept parameters
// - See how argument values are passed to parameters

fn main() {
    // Call `another_function` with an integer argument
    another_function(4);
}

// `another_function` takes a single `i32` parameter and prints it
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}