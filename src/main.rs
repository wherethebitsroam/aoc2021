use std::fs::File;
use std::io::Read;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod util;

pub trait Day {
    fn new(s: &str) -> Self;
    fn part1(&self) -> usize;
    fn part2(&self) -> usize;
}

fn main() {
    let mut f = File::open("../day6.txt").expect("file not found");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("could not read file");

    let result = day6::Day6::new(&buffer).part2();
    println!("result: {}", result);
}
