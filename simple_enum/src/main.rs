#![allow(dead_code, unused_variables)]
fn main() {
    let v1 = Kind::V1(1, -1);
    let v2 = Kind::V2(String::from("Hello, Rust!"));
    let v3 = Kind::V3(1, 1, -1);

    print(v1);
    print(v2);
    print(v3)
}

enum Kind {
    V1(i32, i32),
    V2(String),
    V3(u32, u32, i32),
}

fn print(kind: Kind) {
    match kind {
        Kind::V1(x, y) => println!("Variant 1: {}, {}", x, y),
        Kind::V2(x) => println!("Variant 2: {}", x),
        Kind::V3(x, y, z) => println!("Variant 3: {}, {}, {}", x, y, z),
    }
}
