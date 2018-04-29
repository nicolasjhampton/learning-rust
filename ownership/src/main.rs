fn main() {
    // s not yet declared and thus, invalid (non-hoisted)
    let s = "hello"; // s is valid from this point forward

    string_on_heap();

    var_ownership();

    cloning_heap_data();

    func_ownership();

} // bracket marks end of scope? s no longer valid. Drop called here

fn string_on_heap() {

    // This heap string has an unfixed size and 
    // can be mutated
    let mut heap_string = String::from("hello");

    // String type can be mutated, string literals cannot
    heap_string.push_str(", world!");

    println!("{}", heap_string);
}


fn var_ownership() {
    // integers with fixed, known values.
    // two separate variables pushed onto the stack
    // y makes a copy of x
    let x = 5;
    let y = x;

    // s1 is an unknown size at compile time, thus is heap memory
    // string is made of three values: pointer, length, and capacity
    let s1 = String::from("hello");
    // when we copy the string, we copy the pointer, length, and capacity
    // but the data on the heap the pointer points to stays the same
    // thus s1 and s2 point to the same data on the heap (ownership questions?)

    let s2 = s1; // s1 is "moved" to s2

    // This could cause a "double free" issue at the end of this scope
    // when rust attempts to call 'drop' on s1 and s2
    // to prevent this double free situation that could cause memory
    // corruption, rust considers s1 "no longer valid" after the 
    // assignment to s2, and s1 as a variable will no longer work
    println!("{}", s2);
    println!("{}", s2);
    // println!("{}", s1); // could not compile: 'ownership'! value used here after move

    // because s1 is invalidated after being moved to s2,
    // only s2 is responsible for freeing the memory with drop,
    // and we have no "double free" issues
}

// rust never "automatically" deep copies your data,
// so all default data copies can be assumed to be 
// inexpensive in terms of runtime performance.


// Note: difference between heap data of a string (the actual text in the heap)
// and the stack data of the string (pointer, length, and capacity in the stack)

fn cloning_heap_data() {

    let s1 = String::from("hello");
    let s2 = s1.clone(); // clone method on String types makes a deep copy of the heap
    // copy allows us to assign s1 to s2 without invalidating s1
    println!("s1 = {}, s2 = {}", s1, s2);

    // .clone can be used as a visual sign of an expensive operation taking place
}

fn stack_data_no_clone_needed() {

    // the value of 5, x, and y can all be known at compile time
    // and thus, will never need space on the heap, only the stack.
    // So hewre, x and y will be valid without a clone, and no variable
    // will be invalidated to avoid double freeing on the heap
    let x = 5;
    let y = x;
    // integers and other types like it that can completely exist on 
    // the stack can have a "Copy" trait to indicate that older variables
    // are still usable after reassignment. You can't implement a Copy
    // trait on a type with a drop trait implemented

    println!("x = {}, y = {}", x, y);
}

fn func_ownership() {
    let s = String::from("hello"); 

    takes_ownership(s); // passing heap ownership of s to func
    // println!("{}", s); // s no longer valid in this scope, error

    let x = 5; // Copy trait implemented, implying value only on stack

    makes_copy(x); // x still exists in this scope 
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // drop called on some_string here, freeing memory

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer has no drop implementation. Not needed, not called
