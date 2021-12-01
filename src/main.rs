use std::path::Path;

mod day1;
mod util;

pub trait Day {
    fn new<P: AsRef<Path>>(path: P) -> Self;
    fn part1(&self);
    fn part2(&self);
}

fn main() {
    day1::Day1::new("../day1.txt").part2();
}
