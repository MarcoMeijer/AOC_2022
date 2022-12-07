pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

use std::fs;

use day7::*;

fn main() {
    let file = fs::read_to_string("./in.txt").unwrap();
    println!("{}", day7_gold(&file));
}
