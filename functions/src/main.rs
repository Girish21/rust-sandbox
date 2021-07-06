fn main() {
    let x = return_value(10_000);
    println!("Function: {}", x);
}

// fn five() -> i32 {
//     5
// }

fn return_value(x: i32) -> i32 {
    x
}
