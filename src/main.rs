use std::path::Path;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod util;

pub trait Day {
    fn new<P: AsRef<Path>>(path: P) -> Self;
    fn part1(&self) -> i32;
    fn part2(&self) -> i32;
}

fn main() {
    let result = day5::Day5::new("../day5.txt").part2();
    println!("result: {}", result);
}
