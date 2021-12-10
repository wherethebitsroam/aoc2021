use crate::Day;
use std::collections::HashMap;

// size
// 0: 6
// 1: 2
// 2: 5
// 3: 5
// 4: 4
// 5: 5
// 6: 6
// 7: 3
// 8: 7
// 9: 6

// size inverted
// 2: 1
// 3: 7
// 4: 4
// 5: 2, 3, 5
// 6: 0, 6, 9
// 7: 8

// counts
// a: 8
// b: 6
// c: 8
// d: 7
// e: 4
// f: 9
// g: 7

// counts inverted
// 4: e
// 6: b
// 7: d, g -> d in 4
// 8: a, c -> c in 4
// 9: f

fn digits_to_usize(d: &[usize]) -> usize {
    let mut total = 0;
    let base: usize = 10;
    for (i, x) in d.iter().rev().enumerate() {
        total += x * base.pow(i as u32)
    }
    total
}

struct Line {
    input: Vec<String>,
    output: Vec<String>,
}

impl Line {
    fn parse(line: &str) -> Self {
        let parts: Vec<&str> = line.split(" | ").collect();
        if parts.len() != 2 {
            panic!("Wrong number of parts")
        }
        Self {
            input: parse_sort(parts[0]),
            output: parse_sort(parts[1]),
        }
    }

    fn solve(&self) -> usize {
        // we can determine which character was supposed to be represented
        // based on the count and whether is is part of four
        // counts: actual letter
        // 4: e
        // 6: b
        // 7: d or g -> d in 4
        // 8: a or c -> c in 4
        // 9: f

        // get the letters in four in the input set
        let four: Vec<char> = self
            .input
            .iter()
            .find(|s| s.len() == 4)
            .unwrap()
            .chars()
            .collect();

        // get the counts for each letter in the input
        let mut counts: HashMap<char, usize> = HashMap::new();
        for s in self.input.iter() {
            for c in s.chars() {
                counts.entry(c).and_modify(|x| *x += 1).or_insert(1);
            }
        }

        // work out the translation table for input set to actual
        let mut translate: HashMap<char, char> = HashMap::new();
        for (k, v) in counts.iter() {
            match v {
                4 => translate.insert(*k, 'e'),
                6 => translate.insert(*k, 'b'),
                9 => translate.insert(*k, 'f'),
                7 => {
                    if four.contains(k) {
                        translate.insert(*k, 'd')
                    } else {
                        translate.insert(*k, 'g')
                    }
                }
                8 => {
                    if four.contains(k) {
                        translate.insert(*k, 'c')
                    } else {
                        translate.insert(*k, 'a')
                    }
                }
                n => panic!("Unexpected count: {} for {}", n, k),
            };
        }

        // do the translation
        let translated: Vec<String> = self
            .output
            .iter()
            .map(|s| {
                // translate each char and sort the resulting string
                let mut cs: Vec<char> = s
                    .chars()
                    .map(|c| translate.get(&c).expect("argh!").to_owned())
                    .collect();
                cs.sort_unstable();
                cs.into_iter().collect()
            })
            .collect();

        // map the resulting strings to digits
        let lookup: HashMap<&str, usize> = HashMap::from([
            ("abcefg", 0),
            ("cf", 1),
            ("acdeg", 2),
            ("acdfg", 3),
            ("bcdf", 4),
            ("abdfg", 5),
            ("abdefg", 6),
            ("acf", 7),
            ("abcdefg", 8),
            ("abcdfg", 9),
        ]);

        let digits: Vec<usize> = translated
            .iter()
            .map(|s| *lookup.get(s.as_str()).unwrap())
            .collect();

        digits_to_usize(&digits)
    }
}

pub struct Day8 {
    lines: Vec<Line>,
}

fn parse_sort(s: &str) -> Vec<String> {
    s.split_whitespace()
        .map(|s| {
            let mut c: Vec<char> = s.chars().collect();
            c.sort_unstable();
            c.into_iter().collect()
        })
        .collect()
}

impl Day for Day8 {
    fn new(s: &str) -> Self {
        let lines = s.trim().split('\n').map(|line| Line::parse(line)).collect();
        Self { lines }
    }

    fn part1(&self) -> usize {
        let mut count = 0;
        for line in self.lines.iter() {
            count += line
                .output
                .iter()
                .map(|s| s.len())
                .filter(|l| [2, 3, 4, 7].contains(l))
                .count();
        }

        count
    }

    fn part2(&self) -> usize {
        self.lines.iter().map(|line| line.solve()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    const INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

    #[test]
    fn test_part1_day8() {
        let x = Day8::new(INPUT);
        assert_eq!(26, x.part1());
    }

    #[test]
    fn solver() {
        let line = Line::parse(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        assert_eq!(5353, line.solve());
    }

    #[test]
    fn dig() {
        assert_eq!(1234, digits_to_usize(&[1, 2, 3, 4]))
    }

    #[test]
    fn test_part2_day8() {
        let x = Day8::new(INPUT);
        assert_eq!(61229, x.part2());
    }
}
