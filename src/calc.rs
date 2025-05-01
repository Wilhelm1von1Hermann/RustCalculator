// this is where the calculator logic is implemented

#![allow(dead_code)]
use clearscreen;

fn main() {
    println!("I feel like you should use other functions.");
}

pub fn plus(a: i32, b: i32) { // plus function
    clearscreen::clear().unwrap();
    println!("plus() result: {}", a + b);
    print!("\n"); // new line
}

pub fn minus(a: i32, b: i32) { // minus function
    clearscreen::clear().unwrap();
    println!("minus() result: {}", a - b);
    print!("\n"); // new line
}

pub fn multiplication(a: i32, b: i32) { // multiplication function
    clearscreen::clear().unwrap();
    println!("multiplication() result: {}", a * b);
    print!("\n"); // new line
}

pub fn division(a: f32, b: f32) -> u8 { // division function
    if a == 0.0 || b == 0.0 {
        clearscreen::clear().unwrap();
        println!("Division by zero. Panicking.");
        print!("\n");
        return 1;
    }
    else {
        clearscreen::clear().unwrap();
        println!("division() result: {}", a / b);
        print!("\n"); // new line
        return 0;
    }
}