fn main() {
    println!("Hello, world!");

    another_function();
    another_function2(5);
    another_function3(5, 6);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };
    // Expressions do not include ending semicolons. 
    // If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value. 
    // Keep this in mind as you explore function return values and expressions next.
    println!("The value of y is: {}", y);

    let x = five();

    println!("The value of x is: {}", x);

    let x = plus_one(5);

    println!("The value of x is: {}", x);


}
//Function definitions in Rust start with fn and have a set of parentheses after the function name. 
//The curly brackets tell the compiler where the function body begins and ends.
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

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
