pub mod day1;
pub mod day2;
pub mod day3;

use std::fs;

use day3::*;

fn main() {
    let file = fs::read_to_string("./in.txt").unwrap();
    println!("{}", day3_silver(&file));
}
