// use crate reference_and_borrowing::reference_and_borrowin;
// mod ownership;

fn main() {
    let s = String::from("Hello world!");
    println!("Hello, world!");
    println!("First space is at => {}", first_word(&s));
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}!", hello, world);
    println!("First word is => {}", first_word_slice(&s))
}
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
