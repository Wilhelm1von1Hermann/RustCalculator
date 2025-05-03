// this is where gui is implemented
#![allow(unused_imports)]
use crate::misc; // misc.rs for error handling and exiting
use crate::calc; // calc.rs for calculator logic

pub fn main() { // for loading up the gui
    use clearscreen::*;
    clearscreen::clear().unwrap();
    println!("GUI main() active. Glory to Rust.");
    misc::exit();
}