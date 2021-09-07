#![allow(dead_code, unused_variables)]

fn main() {
    let a: [i32; 3] = [1, 2, 3];
    let mut arr: Vec<VecType> = Vec::new();

    arr.push(VecType::V2(10));
    arr.push(VecType::V3(10.5));
    arr.push(VecType::V1(String::from("Hello Vector")));

    // let copy = &arr;
    let mut_copy = &mut arr;
    mut_copy.push(VecType::V1(String::from("Pushed via borrow")));
    push(mut_copy);

    for elem in mut_copy.iter() {
        match elem {
            VecType::V1(x) => println!("It's a string: {}", x),
            VecType::V2(x) => println!("It's a number: {}", x),
            VecType::V3(x) => println!("It's a floating point number: {}", x),
        }
    }
}

fn push(vec: &mut Vec<VecType>) {
    vec.push(VecType::V1(String::from("from function")));
}

enum VecType {
    V1(String),
    V2(i32),
    V3(f32),
}
