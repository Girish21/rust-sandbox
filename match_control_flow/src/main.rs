fn main() {
    let x = match Size::Small(Variant::XS) {
        Size::Large => 10,
        Size::Medium => 5,
        Size::Small(variant) => {
            println!("{:#?}", variant);
            1
        }
    };
    println!("{}", x);
}

fn add_some(x: Option<i32>, val: i32) -> i32 {
    match x {
        None => val,
        Some(i) => i + val,
    }
}

fn inline_if_let() {
    let x = Size::Large;
    let x = if let Size::Large = x {
        "Large"
    } else {
        "Unknown"
    };
    println!("{}", x);
}

#[derive(Debug)]
enum Variant {
    Normal,
    XS,
    XXS,
}

enum Size {
    Large,
    Medium,
    Small(Variant),
}
