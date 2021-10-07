fn main() {
    // let mut s = String::from("hello");

    // s.push_str(", world!"); // push_str() appends a literal to a String

    // println!("{}", s); // This will print `hello, world!`

    let s = String::from("hello"); // s is valid from this point forward

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s.clone());             // s's value moves into the function...
    println!("{}", s);

    let s1 = String::from("hello");

    let (s1, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s1, len);

}
fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}