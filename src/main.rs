fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len); // hello 5

    let mut s = String::from("hello");
    change(&mut s);
    println!("The value of s is: {}.", s); // hello, world

    let mut s1 = String::from("hello");
    let r1 = &s1;
    let r2 = &s1;
    // let r3 = &mut s1; // CANNOT BORROW MUTABLE SINCE ALREADY BORROWED
    // change(r3);
    // println!("{}, {}, and {}", r1, r2, r3);
    println!("{} and {}", r1, r2); // hello hello

    let hello = "hello";
    println!("The value of hello is {}.", hello); // hello

    let mut phrase = String::from("hello world!");
    let first = first_word(&phrase);
    println!("The value of first is {}.", first); // hello
    // phrase.clear(); // CANNOT BORROW AS MUTABLE
    println!("The value of first is {}.", first); // hello
}

fn calculate_length(s: &String) -> usize {
    let len = s.len();
    // s.push_str(", world!"); // CANNOT BORROW AS MUTABLE
    len
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
/*
fn dangle() -> &String { // RETURNS DANGLING REFERENCE
    let s = String::from("hello");
    &s
}
*/

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
