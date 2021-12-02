use std::path::Path;

mod day1;
mod day2;
mod util;

pub trait Day {
    fn new<P: AsRef<Path>>(path: P) -> Self;
    fn part1(&self);
    fn part2(&self);
}

fn main() {
    day2::Day2::new("../day2.txt").part2();
}
