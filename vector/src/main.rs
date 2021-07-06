enum VecType {
    Int(i32),
    Text(String),
    Float(f32),
}

fn main() {
    let mut v: Vec<VecType> = Vec::new();

    v.push(VecType::Text(String::from("Hello")));
    v.push(VecType::Int(10));
    v.push(VecType::Float(10.0));

    for el in &v {
        match el {
            VecType::Text(t) => println!("Type text: {}", t),
            VecType::Int(t) => println!("Type Int: {}", t),
            VecType::Float(t) => println!("Type Float: {}", t),
        }
    }
}
