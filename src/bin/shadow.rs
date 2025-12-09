// now we will see a concept of shadowing which is when we redeclare the variable and use same variable name in the value

fn main(){
    let x = 3 ; 
    let x =  x + 3;

    println!("Value of x is {x}");
    // this will print "Value of x is 6"

    // but we can't do 
    //  x = x + 3;
    // which throws an error of "cannot mutate immutable variable `x`"
}