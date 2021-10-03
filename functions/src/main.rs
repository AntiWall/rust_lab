fn main() {
    println!("Hello, world!");

    another_function();
    another_function2(5);
    another_function3(5, 6);
}
//Function definitions in Rust start with fn and have a set of parentheses after the function name. 
//The curly brackets tell the compiler where the function body begins and ends.

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function3(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
