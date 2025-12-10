// Simple function call example
//
// Learning goals:
// - Understand how to define and call functions

fn main() {
    println!("This is the main function");
    // Call a second function defined below
    another_function();
}

fn another_function() {
    println!("This is another function being called from main");
}