use crate::util;
use crate::Day;
use std::fmt::Debug;
use std::path::Path;

const WINNERS: [u32; 10] = [
    // rows
    0b1_1111_0000_0000_0000_0000_0000,
    0b0_0000_1111_1000_0000_0000_0000,
    0b0_0000_0000_0111_1100_0000_0000,
    0b0_0000_0000_0000_0011_1110_0000,
    0b0_0000_0000_0000_0000_0001_1111,
    // columns
    0b1_0000_1000_0100_0010_0001_0000,
    0b0_1000_0100_0010_0001_0000_1000,
    0b0_0100_0010_0001_0000_1000_0100,
    0b0_0010_0001_0000_1000_0100_0010,
    0b0_0001_0000_1000_0100_0010_0001,
];

#[derive(Debug)]
struct Board {
    // bitmask
    checked: u32,
    values: [i32; 25],
}

impl Board {
    fn check(&mut self, i: usize) {
        self.checked |= 1 << i
    }

    fn is_checked(&self, i: usize) -> bool {
        self.checked & 1 << i > 0
    }

    fn mark(&mut self, value: i32) {
        for i in 0..self.values.len() {
            if self.values[i] == value {
                self.check(i);
            }
        }
    }

    fn has_won(&self) -> bool {
        WINNERS.iter().any(|w| *w & self.checked == *w)
    }

    fn sum_unchecked(&self) -> i32 {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(i, v)| if self.is_checked(i) { None } else { Some(v) })
            .sum()
    }
}

fn parse(lines: &[String]) -> (Vec<i32>, Vec<Board>) {
    let draw = lines[0]
        .split(',')
        .map(|n| n.parse().expect("invalid number"))
        .collect();

    let mut i = 1;
    let mut boards = Vec::new();
    while i + 6 < lines.len() {
        // skip newline
        i += 1;
        let mut values = [0; 25];
        for row in 0..5 {
            let r0: Vec<i32> = lines[i]
                .split_whitespace()
                .map(|n| n.parse().expect("invalid number"))
                .collect();
            for col in 0..5 {
                values[row * 5 + col] = r0[col];
            }
            i += 1;
        }
        boards.push(Board { checked: 0, values })
    }
    (draw, boards)
}

pub struct Day4 {
    lines: Vec<String>,
}

impl Day4 {}

impl Day for Day4 {
    fn new<P: AsRef<Path>>(path: P) -> Self {
        let lines = util::read_lines(path).expect("bad input");
        Self { lines }
    }

    fn part1(&self) -> i32 {
        let (draw, mut boards) = parse(&self.lines);
        for v in draw {
            for b in boards.iter_mut() {
                b.mark(v);
                if b.has_won() {
                    let unchecked = b.sum_unchecked();
                    return unchecked * v;
                }
            }
        }

        0
    }

    fn part2(&self) -> i32 {
        let (draw, mut boards) = parse(&self.lines);
        let mut remaining: Vec<_> = (0..boards.len()).collect();
        for v in draw {
            for b in boards.iter_mut() {
                b.mark(v);
            }
            if remaining.len() == 1 && boards[remaining[0]].has_won() {
                let unchecked = boards[remaining[0]].sum_unchecked();
                return unchecked * v;
            }
            // remove the completed boards from remaining
            remaining = remaining
                .into_iter()
                .filter(|&i| !boards[i].has_won())
                .collect();
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

    #[test]
    fn test_parse() {
        let lines: Vec<String> = INPUT.split('\n').map(|s| s.to_owned()).collect();
        let (_, boards) = parse(&lines);
        for b in boards {
            println!("{:?}", b.values);
        }
    }

    #[test]
    fn test_part1() {
        let lines: Vec<String> = INPUT.split('\n').map(|s| s.to_owned()).collect();
        assert_eq!(4512, Day4 { lines }.part1());
    }

    #[test]
    fn test_part2() {
        let lines: Vec<String> = INPUT.split('\n').map(|s| s.to_owned()).collect();
        assert_eq!(1924, Day4 { lines }.part2());
    }
}
