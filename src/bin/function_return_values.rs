// Function return values example
//
// Learning goals:
// - Return values from functions using `->` and expression return
// - Note: omitting the terminating semicolon returns the expression

// Returns an `i32` value. The final expression `5` is returned here.
fn five() -> i32 {
    5
}

fn main() {
    let x = five();
    println!("The value of x is {x}");
}