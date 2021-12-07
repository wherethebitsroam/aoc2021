use crate::Day;

pub struct Day6 {
    counter: [usize; 9],
}

impl Day for Day6 {
    fn new(s: &str) -> Self {
        let mut counter = [0; 9];
        for n in s.trim_end().split(',') {
            let n: usize = n.parse().expect("invalid number");
            counter[n] += 1;
        }
        Self { counter }
    }

    fn part1(&self) -> usize {
        let mut counter = self.counter;

        for i in 0..80 {
            let v = counter[i % 9];
            counter[(i + 7) % 9] += v;
        }

        counter.iter().sum()
    }

    fn part2(&self) -> usize {
        let mut counter = self.counter;

        for i in 0..256 {
            let v = counter[i % 9];
            counter[(i + 7) % 9] += v;
        }

        counter.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_parse() {
        let x = Day6::new(INPUT);
        assert_eq!(&[0, 1, 1, 2, 1, 0, 0, 0, 0], &x.counter);
    }

    #[test]
    fn test_day6_part1() {
        let x = Day6::new(INPUT);
        assert_eq!(5934, x.part1());
    }
}
