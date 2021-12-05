use crate::util;
use crate::Day;
use std::path::Path;

pub struct Day1 {
    v: Vec<i32>,
}

impl Day for Day1 {
    fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            v: util::read_input(path).expect("read input"),
        }
    }

    fn part1(&self) -> i32 {
        let v = &self.v;
        let mut increases = 0;
        for i in 1..v.len() {
            if v[i] > v[i - 1] {
                increases += 1;
            }
        }

        increases
    }

    fn part2(&self) -> i32 {
        let v = &self.v;
        let mut increases = 0;
        for i in 3..v.len() {
            let old = v[i - 1] + v[i - 2] + v[i - 3];
            let new = v[i] + v[i - 1] + v[i - 2];
            if new > old {
                increases += 1;
            }
        }

        increases
    }
}
