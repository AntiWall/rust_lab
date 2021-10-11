fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}
fn main() {
    let mut s = String::from("hello 114 514");
    let s1 = first_word(&s);
    println!("{}.", s1);
    // s.clear();

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    println!("{}", slice);

    let slice = &s[0..4];
    println!("{}", slice);

}