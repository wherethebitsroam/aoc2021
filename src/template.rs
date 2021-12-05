use crate::util;
use crate::Day;
use std::path::Path;

pub struct DayX {
    lines: Vec<String>,
}

impl Day for DayX {
    fn new<P: AsRef<Path>>(path: P) -> Self {
        let lines = util::read_lines(path).expect("bad input");
        Self { lines }
    }

    fn part1(&self) -> i32 {
        todo!()
    }

    fn part2(&self) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    const INPUT: &str = "";

    #[test]
    fn test_parse() {
        let lines: Vec<String> = INPUT.split('\n').map(|s| s.to_owned()).collect();
    }
}
