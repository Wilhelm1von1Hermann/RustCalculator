// this is where the calculator logic is implemented

#![allow(dead_code)]
use clearscreen::{self};

fn main() { // why did i keep this
    println!("I feel like you should use other functions.");
}

pub fn plus(a: f32, b: f32) { // plus function
    clearscreen::clear().unwrap();
    println!("plus() result: {}", a + b);
    print!("\n"); // new line
}

pub fn minus(a: f32, b: f32) { // minus function
    clearscreen::clear().unwrap();
    println!("minus() result: {}", a - b);
    print!("\n"); // new line
}

pub fn multiplication(a: f32, b: f32) { // multiplication function
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

pub fn remainder(a: f32, b: f32) { // remainder function
    clearscreen::clear().unwrap();
    println!("remainder() result: {}", a % b);
    print!("\n");
}

pub fn squares(side: f32) -> u8 { // square's area and circumference function
    clearscreen::clear().unwrap();
    if side > 0.0 {
        println!("squares() AREA result: {}", side*side * 4.0);
        println!("squares() CIRCUMFERENCE result: {}", side * 4.0);
        print!("\n");
        return 0; // for main.rs to print "why did you set radius than 0"
    }
    else {
        println!("The side length is smaller than 0. Can't proceed.");
        print!("\n");
        return 1;
    }
}

pub fn circles(radius: f32) -> u8 { // circle's area and circumference function
    clearscreen::clear().unwrap();
    const PI: f32 = 3.1415926;
    if radius > 0.0 {
        println!("circles() AREA result: {}", radius*radius * PI);
        println!("circles() CIRCUMFERENCE result: {}", radius * PI);
        print!("\n");
        return 0;
    }
    else {
        println!("Radius set to 0 or less. Can't proceed.");
        print!("\n");
        return 1;
    }
}