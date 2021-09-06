#![allow(dead_code, unused_variables)]
fn main() {
    let v1 = Kind::V1(1, -1);
    let v2 = Kind::V2(String::from("Hello, Rust!"));
    let v3 = Kind::V3(1, 1, -1);

    let m1 = Message::Write(String::from("Hello"));

    print(v1);
    print(v2);
    print(v3);
    print_message(m1);
}

enum Kind {
    V1(i32, i32),
    V2(String),
    V3(u32, u32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn print(kind: Kind) {
    match kind {
        Kind::V1(x, y) => println!("Variant 1: {}, {}", x, y),
        Kind::V2(x) => println!("Variant 2: {}", x),
        Kind::V3(x, y, z) => println!("Variant 3: {}, {}, {}", x, y, z),
    }
}

fn print_message(message: Message) {
    match message {
        Message::ChangeColor(r, g, b) => println!("new Color: RGB({}, {}, {})", r, g, b),
        Message::Move { x, y } => println!("Move to: ({}, {})", x, y),
        Message::Quit => println!("Quit"),
        Message::Write(val) => println!("Message: {}", val),
    }
}
