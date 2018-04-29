fn main() {
    // let mut number = 3;

    // while number != 0 {
    //     println!("{}!", number);

    //     number = number - 1;
    // }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");

    for_loop();
}

// fn for_loop_while() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);

//         index = index + 1;
//     }
// }

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
