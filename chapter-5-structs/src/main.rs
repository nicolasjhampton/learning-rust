#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.height * self.width
    }

    fn can_hold(&self, inner: &Rectangle) -> bool {
        self.width >= inner.width && self.height >= inner.height
    }

    fn square(size: i32) -> Rectangle {
        Rectangle{ width: size, height: size }
    }
}

fn main() {
    let shape = Rectangle{ width: 30, height: 50 };
    let inner_shape = Rectangle{ width: 25, height: 51 };
    // let sq_ft = area(&shape);
    let sq_ft = shape.area();
    println!("The rectangle has an area of {} square feet.", sq_ft);
    println!("Rectangle 2 can hold rectangle 1: {}", shape.can_hold(&inner_shape));
    let square = Rectangle::square(5);
    println!("This is a square made with the asso. function on Rectangle: {:?}", square);
    println!("{:?}", shape);
    println!("Verbose: {:#?}", shape);
}

fn area(rect: &Rectangle) -> i32 {
    rect.height * rect.width
}