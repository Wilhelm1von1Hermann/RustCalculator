#![allow(dead_code)]
#![allow(unused_imports)]

use inline_colorization::*;

pub fn throw_error(error: &str) { // function for throwing errors
    println!("{color_red}{error}{color_reset}");
}
pub fn exit() { // function for exiting
    use std::io::stdin;
    const EXIT_LINE2: &str = "Thanks for using the calculator. Press Enter to exit.";
    print!("\n");
    println!("{color_bright_blue}{EXIT_LINE2}{color_reset}");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
pub fn exit_line() { // function for other function's completion
    const EXIT_LINE: &str = "Thanks for using the calculator.";
    print!("\n");
    println!("{color_bright_blue}{EXIT_LINE}{color_reset}");
}