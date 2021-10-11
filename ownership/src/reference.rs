// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
    
//     let s2 = String::from("hello");

//     let len = calculate_length2(s2);
//     println!("The length of '{}' is {}.", s2, len);

// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
// fn calculate_length2(s: String) -> usize {
//     s.len()
// }

// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}.", s);

    let reference_to_nothing = dangle();

}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
