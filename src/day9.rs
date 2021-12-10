use crate::Day;
use std::collections::HashSet;

pub struct Day9 {
    levels: Vec<Vec<usize>>,
}

impl Day for Day9 {
    fn new(s: &str) -> Self {
        let levels = s
            .trim()
            .split('\n')
            .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
            .collect();
        Self { levels }
    }

    fn part1(&self) -> usize {
        let mut lowest: Vec<usize> = Vec::new();
        for i in 0..self.levels.len() {
            let row = &self.levels[i];
            for j in 0..row.len() {
                let x = row[j];
                let all_lower = (i == 0 || x < self.levels[i - 1][j])
                    && (i == self.levels.len() - 1 || x < self.levels[i + 1][j])
                    && (j == 0 || x < self.levels[i][j - 1])
                    && (j == row.len() - 1 || x < self.levels[i][j + 1]);
                if all_lower {
                    lowest.push(x);
                }
            }
        }

        lowest.iter().sum::<usize>() + lowest.len()
    }

    fn part2(&self) -> usize {
        // make sets of the non-9 levels and their non-9 neighbours
        let mut sets = Vec::new();
        for (i, row) in self.levels.iter().enumerate() {
            for j in 0..row.len() {
                if self.levels[i][j] != 9 {
                    let mut set = HashSet::from([(i, j)]);
                    if i > 0 && self.levels[i - 1][j] != 9 {
                        set.insert((i - 1, j));
                    }
                    if j > 0 && self.levels[i][j - 1] != 9 {
                        set.insert((i, j - 1));
                    }
                    sets.push(set);
                }
            }
        }

        // join overlapping sets into basins
        let mut basins = Vec::new();

        while !sets.is_empty() {
            let mut set = sets.pop().unwrap();

            let (linked, mut rest): (Vec<_>, Vec<_>) =
                sets.into_iter().partition(|s| !s.is_disjoint(&set));

            if linked.is_empty() {
                // we've found everything that links to this set. Add it as a basin
                basins.push(set);
            } else {
                // merge the linked sets and add it back
                for link in linked.into_iter() {
                    set = &set | &link
                }
                rest.push(set);
            }
            sets = rest;
        }

        // get the 3 biggest basins and the product of their sizes
        let mut sizes: Vec<_> = basins.iter().map(|b| b.len()).collect();
        sizes.sort_unstable();
        sizes.iter().rev().take(3).product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_day9_part1() {
        let x = Day9::new(INPUT);
        assert_eq!(15, x.part1());
    }

    #[test]
    fn test_day9_part2() {
        let x = Day9::new(INPUT);
        assert_eq!(1134, x.part2());
    }
}
