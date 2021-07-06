use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter string: ");
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(err) => panic!("Error! {}", err),
    }

    let word = first_word(&input.trim());

    println!("First word: {}", word);
}

fn first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
