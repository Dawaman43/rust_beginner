// if we want function to return some value we must use -> function
// and we shouldn't use semicolon in return values it will throw an error
fn five() -> i32{
    5
}

fn main() {
    let x = five();
    println!("The value of x is {x}")
}