// this is just the interface and some small logic
mod calc; // importing calc.rs for logic

fn main() { // boots up the calculator
    let ver = "2025-05-01 v0.0.1";
    println!("main() active, welcome to Rust.");
    println!("CALC VERSION - {}", ver);
    calc();
}

fn calc() { // the cli and some parts of the calculator
    use std::io::stdin;

    println!("\n \n"); // skips two lines after main()
    println!("Select your operation.");
    println!("1 - Plus; 2 - Minus; 3 - Multiplication; 4 - Division; 0 - EXIT."); // all variants are here
    const EXIT_LINE: &str = "All good! Thanks for using the calculator.";
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).expect("Failed to read line");

    let input = user_input.trim();

    if input == "1" { // plus
        println!("Plus selected! First number:");
        let mut a = String::new();
        stdin().read_line(&mut a).expect("fail.");
        let a: i32 = a.trim().parse().expect("fail. you need a number!");
        println!("Second number:");
        let mut b = String::new();
        stdin().read_line(&mut b).expect("fail.");
        let b: i32 = b.trim().parse().expect("fail. you need a number!");

        calc::plus(a, b);
        println!("{}", EXIT_LINE);
        std::io::stdin().read_line(&mut String::new()).unwrap();
    }
    else if input == "2" { // minus
        println!("Minus selected! Your first number:");
        let mut a = String::new();
        stdin().read_line(&mut a).expect("fail.");
        let a: i32 = a.trim().parse().expect("fail. you need a number!");
        println!("Second number:");
        let mut b = String::new();
        stdin().read_line(&mut b).expect("fail.");
        let b: i32 = b.trim().parse().expect("fail. you need a number!");

        calc::minus(a, b);
        println!("{}", EXIT_LINE);
        std::io::stdin().read_line(&mut String::new()).unwrap();
    }
    else if input == "3" { // multiplication
        println!("Multiplication selected! Your first number:");
        let mut a = String::new();
        stdin().read_line(&mut a).expect("fail.");
        let a: i32 = a.trim().parse().expect("fail. you need a number!");
        println!("Second number:");
        let mut b = String::new();
        stdin().read_line(&mut b).expect("fail.");
        let b: i32 = b.trim().parse().expect("fail. you need a number!");

        calc::multiplication(a, b);
        println!("{}", EXIT_LINE);
        std::io::stdin().read_line(&mut String::new()).unwrap();
    }
    else if input == "4" { // division
        println!("Division selected. Your first number:");
        let mut a = String::new();
        stdin().read_line(&mut a).expect("fail.");
        let a: f32 = a.trim().parse().expect("fail. you need a number!");
        println!("Second number:");
        let mut b = String::new();
        stdin().read_line(&mut b).expect("fail.");
        let b: f32 = b.trim().parse().expect("fail. you need a number!");

        calc::division(a, b);
        let n: u8 = calc::division(a, b);
        if n == 1 {
            println!("I'm sorry? Dividing by zero? Stonks.");
        }
        else {
            println!("{}", EXIT_LINE);
        }
        std::io::stdin().read_line(&mut String::new()).unwrap();
    }
    else if input == "0" {

    }
    else {
        println!("Hasn't been able to find an operator of that kind. Try again.");
        calc();
    }
 }