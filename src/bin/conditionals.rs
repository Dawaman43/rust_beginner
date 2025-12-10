// we can use conditionals in rust to compute expressions based on conditions

fn main(){
    let x = 3;

    if x > 18 {
        println!("You are an adult")
        
    } else if x == 18 {
        println!("You are 18 years old")
    } else {
        println!("You are less than 18 years old")
    }

    // we can use condtionals in the let assigning too
    let y = true;
    let z = if y {6} else {1};
    println!("The value of y is {z}")
}