use std::io;

fn main() {
    let array: [u16; 5] = [1, 2, 3, 4, 5];
    let mut guess = String::new();

    println!("Enter an index:");

    io::stdin().read_line(&mut guess).expect("Enter an index!");

    let guess: usize = guess.trim().parse().expect("Enter a numner");

    let guess = array[guess];

    println!("The guessed number is {}", guess);
}
