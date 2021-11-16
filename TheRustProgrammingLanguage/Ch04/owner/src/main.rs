fn main() {
    // basic
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(5);

    // returning ownership
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    // reference and borrowing
    let s4 = String::from("hello");
    let len = calculate_length(&s4);

    // mutable reference
    let mut s5 = String::from("hello");
    change(&mut s5);
    /* can only designate one mutable reference (data race)
    let r1 = &mut s5;
    let r2 = &mut s5;
    */

    // dangling
    // let reference_to_nothing = dangle();
    let not_dangling_pointer = no_dangle();

    // string slice
    let s6 = String::from("hello world");
    let hello = &s6[0..5];
    let world = &s6[6..11];

    // let mut s7 = String::from("hello world");
    // let firstword = first_word(&s7);
    // // s7.clear()   // error

    let my_string = String::from("hello world");
    let str_word = first_word(&my_string[..]);

    let my_string_literal = "hello world";
    let lit_word = first_word(&my_string_literal[..]);

    // string literal is also string slice
    let word2 = first_word(my_string_literal);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
/*
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
*/

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

// &str::string slice
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }

    &s[..]
}