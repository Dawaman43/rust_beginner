// we can call another function in the main function

fn main(){
    println!("This is main function");
    another_function();
}

fn another_function(){
    println!("Another function");
}