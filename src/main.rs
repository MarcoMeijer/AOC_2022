pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

use std::fs;

use day4::*;

fn main() {
    let file = fs::read_to_string("./in.txt").unwrap();
    println!("{}", day4_silver(&file));
}
