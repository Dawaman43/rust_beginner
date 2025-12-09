// Variables are like storages that hold values as a storage and they are block scoped or immutable(which means we can not reassign later)
// but there is 1 key word in rust that allows us to reassign and use later for specific purposes 
// that variable is "mut"

fn main(){
    let x : i32 = 3;
    println!("The value of x befor reassigning is {x}");

    // this now prints "The value of x befor reassigning is 3"
    // but if we reassign the value x : i32 = 2;
    // now it returns the error of " ^^^^^ cannot assign twice to immutable variable"

    //now lets demostrate the use of mut keyword
    let mut y : i32 = 4;
    println!("The value of y befor reassigning is {y}");
    // ^^^ this will print "The value of x befor reassigning is 4"

    // and lets reassign it now
    y = 7;
     println!("The value of y after reassigning is {y}");
    // ^^^ this will print "The value of x after reassigning is 7"

    // and another thing to mention in mutablity is we will get warning if we reassign the variable before reading
    let  z  = 9;
    // i commented out the mut because it throws a warning of warning: variable does not need to be mutable

    //let mut z  = 9;

    // and lets reassign it now
    // z = 7;
     println!("The value of z is {z}");
    // ^^^ this will return an error  maybe it is overwritten before being read?
}