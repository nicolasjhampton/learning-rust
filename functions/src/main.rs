fn main() {
    println!("Hello, world!");

    another_function(5, 6);
    block_expression();
    let z = five();
    println!("The value of z is: {}", z);
    println!("The value of add_one(9) is: {}", add_one(9));

}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn block_expression() {
  let a = 5;

  let b = {
    let a = 3;
    a + 1
  };

  println!("The value of a is: {}", a);
  println!("The value of b is: {}", b);
}

fn add_one(x: i32) -> i32 {
  x + 1
}

fn five() -> i32 {
  5
}