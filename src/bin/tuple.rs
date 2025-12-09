// to group different types into one compound we will use tuple
fn main(){
    let tup  = (1, 1.4, "string");
    let (x, y, z) = tup;
    println!("The value of x : {x}");
    // this will print The value of x : 1

    println!("The value of y : {y}");
    // this will print The value of y : 1.4

    println!("The value of z : {z}");
    // this will print The value of z : string

    // and we can access tuple values using period
    println!("The value of second element in tuple is , {}", tup.1);

}