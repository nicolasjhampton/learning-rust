fn main() {
    println!("Hello, world!");

    another_function(5, 6);
    block_expression()
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