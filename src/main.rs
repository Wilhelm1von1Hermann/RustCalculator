// this is just the interface and its logic

mod calc; // importing calc.rs for the calculator logic

fn main() { // boots up the calculator
    let ver = "2025-05-02 v0.0.2";
    println!("main() active, glory to Rust.");
    println!("CALC VERSION - {}", ver);
    calc();
}

fn calc() { // the cli and some parts of the calculator
    use std::io::stdin;
    use inline_colorization::*;

    println!("\n \n"); // skips two lines after main() or failing
    println!("{style_bold}Select your operation.{style_reset}"); // bold
    println!("{style_bold}1 - Plus (+); 2 - Minus (-); 3 - Multiplication (*); 4 - Division (/); 5 - Remainder (%); 0 - EXIT.{style_reset}"); // all variants are here
    const EXIT_LINE: &str = "Thanks for using the calculator.";
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).expect("Failed.");

    let input = match user_input.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("{color_red}Failed. You need a number!{color_reset}");
            return calc();
        }
    };

    if input == 1 { // plus
        println!("Plus selected! First number:");
        let mut a = String::new();
        stdin().read_line(&mut a).expect("fail.");
        let a = match a.trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("{color_red}Failed. You need a number!{color_reset}");
                return calc();
            }
        };

        println!("Second number:");
        let mut b = String::new();
        stdin().read_line(&mut b).expect("fail.");
        let b: f32 = match b.trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("{color_red}Failed. You need a number!{color_reset}");
                return calc();
            }
        };

        calc::plus(a, b);
        println!("{color_bright_blue}{EXIT_LINE}{color_reset}");
        calc();
    }

    else if input == 2 { // minus
        println!("Minus selected! Your first number:");
        let mut a = String::new();
        stdin().read_line(&mut a).expect("fail.");
        let a: f32 = match a.trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("{color_red}Failed. You need a number!{color_reset}");
                return calc();
            }
        };
        println!("Second number:");
        let mut b = String::new();
        stdin().read_line(&mut b).expect("fail.");
        let b: f32 = match b.trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("{color_red}Failed. You need a number!{color_reset}");
                return calc();
            }
        };

        calc::minus(a, b);
        println!("{color_bright_blue}{EXIT_LINE}{color_reset}");
        calc();
    }

    else if input == 3 { // multiplication
        println!("Multiplication selected! Your first number:");
        let mut a = String::new();
        stdin().read_line(&mut a).expect("fail.");
        let a: f32 = match a.trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("{color_red}Failed. You need a number!{color_reset}");
                return calc();
            }
        };
        println!("Second number:");
        let mut b = String::new();
        stdin().read_line(&mut b).expect("fail.");
        let b: f32 = match b.trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("{color_red}Failed. You need a number!{color_reset}");
                return calc();
            }
        };

        calc::multiplication(a, b);
        println!("{color_bright_blue}{EXIT_LINE}{color_reset}");
        calc();
    }

    else if input == 4 { // division
        println!("Division selected. Your first number:");
        let mut a = String::new();
        stdin().read_line(&mut a).expect("fail.");
        let a: f32 = match a.trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("{color_red}Failed. You need a number!{color_reset}");
                return calc();
            }
        };
        println!("Second number:");
        let mut b = String::new();
        stdin().read_line(&mut b).expect("fail.");
        let b: f32 = match b.trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("{color_red}Failed. You need a number!{color_reset}");
                return calc();
            }
        };

        calc::division(a, b);
        let n: u8 = calc::division(a, b);
        if n == 1 {
            println!("{}", "I'm sorry? Dividing by zero? Stonks.");
            calc();
        }
        else {
            println!("{color_bright_blue}{EXIT_LINE}{color_reset}");
        }
        calc();
    }

    else if input == 5 { // remainder function
        println!("Remainder selected. Your first number:");
        let mut a = String::new();
        stdin().read_line(&mut a).unwrap();
        let a: f32 = match a.trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("{color_red}Failed. You need a number.{color_reset}");
                return calc();
            }
        };
        println!("Second number:");
        let mut b = String::new();
        stdin().read_line(&mut b).unwrap();
        let b: f32 = match b.trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("{color_red}Failed. You need a number.{color_reset}");
                return calc();
            }
        };

        calc::remainder(a, b);
        println!("{color_bright_blue}{EXIT_LINE}{color_reset}");
        calc();
    }

    else if input == 0 {
        println!("Exiting... 1337");
    }

    else {
        println!("Hasn't been able to find an operator of that kind. Try again.");
        calc();
    }
 }