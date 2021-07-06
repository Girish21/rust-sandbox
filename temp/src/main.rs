fn main() {
    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    let _v_ref = &v[0];

    v.push(4);

    for el in &v {
        println!("{}", el)
    }
}
