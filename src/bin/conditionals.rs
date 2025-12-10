// Conditionals example
//
// Learning goals for beginners:
// - Understand `if` / `else if` / `else` branching
// - See how conditions control program flow
// - Learn that `if` can be used as an expression when assigning values

fn main() {
    // Example 1: branching with `if` / `else if` / `else`
    let age = 3;

    if age > 18 {
        println!("You are an adult");
    } else if age == 18 {
        println!("You are 18 years old");
    } else {
        println!("You are less than 18 years old");
    }

    // Example 2: using `if` as an expression
    // `if` can return a value; both branches must return the same type.
    let condition = true;
    let number = if condition { 6 } else { 1 };
    println!("The value chosen by the condition is: {number}");
}