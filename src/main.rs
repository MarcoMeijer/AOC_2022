mod day1;

use std::fs;

use day1::day1;

fn main() {
    let file = fs::read_to_string("./in.txt").unwrap();
    println!("{}", day1(&file));
}
