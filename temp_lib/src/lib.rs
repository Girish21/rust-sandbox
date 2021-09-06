#![allow(dead_code)]

mod restaurant;
pub use crate::restaurant::restaurant::Variations;

fn main() {
    let x = Variations::V2(String::from("Hello"));

    match x {
        Variations::V2(a) => println!("{}", a),
        _ => println!("🤷🏻‍♂️"),
    }
}
