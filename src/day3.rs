use crate::util;
use crate::Day;

pub struct Day3 {
    lines: Vec<String>,
}

impl Day for Day3 {
    fn new(s: &str) -> Self {
        Self {
            lines: util::read_lines(s),
        }
    }

    fn part1(&self) -> usize {
        let l = self.lines.len() as u32 / 2;
        let x = self
            .lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<u32>().expect("xx"))
                    .collect::<Vec<_>>()
            })
            .reduce(|acc, item| acc.iter().zip(&item).map(|(a, b)| a + b).collect())
            .expect("there must be lines");

        let g = x
            .iter()
            .map(|v| if *v > l { 1 } else { 0 })
            .collect::<Vec<_>>();
        let e = g
            .iter()
            .map(|x| if *x > 0 { 0 } else { 1 })
            .collect::<Vec<_>>();
        let gamma = binary_to_u32(&g);
        let epsilon = binary_to_u32(&e);

        println!("g={:?}", g);
        println!("e={:?}", e);
        println!(
            "gamma={}, epsilon={}, prod={}",
            gamma,
            epsilon,
            gamma * epsilon
        );

        (gamma * epsilon) as usize
    }

    fn part2(&self) -> usize {
        let x: Vec<Vec<_>> = self
            .lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<u32>().expect("xx"))
                    .collect::<Vec<_>>()
            })
            .collect();

        let mut oxy = x.clone();
        for i in 0..oxy[0].len() {
            let value = most_common(&oxy, i);
            oxy = oxy.into_iter().filter(|row| row[i] == value).collect();
            if oxy.len() == 1 {
                break;
            }
        }

        let mut co2 = x.clone();
        for i in 0..co2[0].len() {
            let value = if most_common(&co2, i) == 1 { 0 } else { 1 };
            co2 = co2.into_iter().filter(|row| row[i] == value).collect();
            if co2.len() == 1 {
                break;
            }
        }

        let oxy_value = binary_to_u32(&oxy[0]);
        let co2_value = binary_to_u32(&co2[0]);

        (oxy_value * co2_value) as usize
    }
}

fn most_common(b: &[Vec<u32>], i: usize) -> u32 {
    let mut one = 0;
    let mut zero = 0;
    for row in b {
        if row[i] == 1 {
            one += 1;
        } else {
            zero += 1;
        }
    }
    if one >= zero {
        1
    } else {
        0
    }
}

fn binary_to_u32(b: &[u32]) -> u32 {
    let mut out = 0;
    for i in 0..b.len() {
        out += b[b.len() - 1 - i] << i
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn bin() {
        assert_eq!(22, binary_to_u32(&[1, 0, 1, 1, 0]))
    }

    #[test]
    fn part1() {
        let s = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let lines: Vec<String> = s.split_whitespace().map(|s| s.to_owned()).collect();
        assert_eq!(Day3 { lines }.part1(), 198);
    }

    #[test]
    fn part2() {
        let s = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let lines: Vec<String> = s.split_whitespace().map(|s| s.to_owned()).collect();
        assert_eq!(Day3 { lines }.part2(), 230);
    }
}
