pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

use std::fs;

use day8::*;

fn main() {
    let file = fs::read_to_string("./in.txt").unwrap();
    println!("{}", day8_gold(&file));
}
