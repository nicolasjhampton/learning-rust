fn main() {
    // let v = vec![1, 2, 3]; // type here is inferred
    // let mut v: Vec<i32> = Vec::new(); // type is needed if no values supplied for inference
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    
    
} // v goes out of scope and is dropped, but so are all it's contents
