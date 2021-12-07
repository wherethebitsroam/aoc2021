use crate::Day;

pub struct Day7 {
    values: Vec<i32>,
}

impl Day for Day7 {
    fn new(s: &str) -> Self {
        let values = s
            .trim()
            .split(',')
            .map(|v| v.parse().expect("not a number"))
            .collect();
        Self { values }
    }

    fn part1(&self) -> usize {
        let min = self.values.iter().min().unwrap();
        let max = self.values.iter().max().unwrap();

        let mut min_cost = None;
        for i in *min..=*max {
            let cost: i32 = self.values.iter().map(|v| (v - i).abs()).sum();
            min_cost = min_cost
                .map(|min| if cost < min { cost } else { min })
                .or(Some(cost))
        }

        min_cost.unwrap() as usize
    }

    fn part2(&self) -> usize {
        let min = self.values.iter().min().unwrap();
        let max = self.values.iter().max().unwrap();

        let mut min_cost = None;
        for i in *min..=*max {
            let cost: i32 = self.values.iter().map(|v| fuel((v - i).abs())).sum();
            min_cost = min_cost
                .map(|min| if cost < min { cost } else { min })
                .or(Some(cost))
        }

        min_cost.unwrap() as usize
    }
}

fn fuel(i: i32) -> i32 {
    i * (i + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_day7_part1() {
        let x = Day7::new(INPUT);
        assert_eq!(37, x.part1());
    }

    #[test]
    fn test_day7_part2() {
        let x = Day7::new(INPUT);
        assert_eq!(168, x.part2());
    }
}
