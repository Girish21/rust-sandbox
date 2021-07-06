#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn square(side: i32) -> Rectangle {
        Rectangle {
            width: side,
            height: side,
        }
    }

    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn increment(&mut self) {
        self.height += 1;
        self.width += 1;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 31,
        height: 31,
    };

    let rect2 = Rectangle {
        width: 32,
        height: 32,
    };

    let rect3 = Rectangle::square(8);

    rect1.increment();

    println!("Square {:?}", rect3);
    println!("{:?}\nArea: {}", rect1, rect1.area());
    println!("Can fit: {}", rect1.can_hold(&rect2));
    println!("Can fit: {}", rect1.can_hold(&rect3));
}
