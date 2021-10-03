use std::io;

fn main() {
    let mut x = 2.0; // f64

    let y: f32 = 3.0; // f32

    println!("{}", x);
    x = 5.2;
    println!("{}", x);
    println!("{}", y);

    // addition
    let sum = 5 + 10;
    println!("{}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("{}", difference);

    // multiplication
    let product = 4 * 30;
    println!("{}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("{}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("{}", remainder);

    let t = true;
    println!("{}", t);

    let f: bool = false; // with explicit type annotation
    println!("{}", f);

    let c = 'z';
    println!("{}", c);

    let z = 'ℤ';
    println!("{}", z);

    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);

    let tup2 = (500, 6.4, 1);

    let (x1, y2, z3) = tup2;

    println!("The value of y is: {}", y2);

    let x2: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x2.0;
    println!("{}", five_hundred);

    let six_point_four = x2.1;
    println!("{}", six_point_four);

    let one = x2.2;
    println!("{}", one);

    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("{:?}", months);

    //Here, i32 is the type of each element. After the semicolon, the number 5 indicates the array contains five elements.
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    //The array named a will contain 5 elements that will all be set to the value 3 initially. 
    //This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.

    let a = [3; 5];
    println!("{:?}", a);

    let first = a[0];
    println!("{:?}", first);

    let second = a[1];
    println!("{:?}", second);

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

}
