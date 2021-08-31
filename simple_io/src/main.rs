use std::io::{self, Write};

fn main() {
    let quit: &str = "f";

    let mut sum = 0;

    let sum = loop {
        let mut input = String::new();
        print!("enter a number ('f' to quit): ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => panic!("enter a string"),
        }
        let input = &input.trim();
        match input.eq(&quit) {
            true => break sum,
            false => {}
        }

        let parsed_number: i32 = match input.trim().parse() {
            Ok(x) => x,
            Err(_) => panic!("enter a number"),
        };

        sum += parsed_number;
    };

    println!("the sum is: {}", sum);
}
