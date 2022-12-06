pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

use std::fs;

use day6::*;

fn main() {
    let file = fs::read_to_string("./in.txt").unwrap();
    println!("{}", day6_gold(&file));
}
