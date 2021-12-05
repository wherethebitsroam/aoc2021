use crate::util;
use crate::Day;
use std::collections::HashMap;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

pub struct Day5 {
    points: Vec<(Point, Point)>,
}

impl Day5 {
    fn parse(lines: &[String]) -> Self {
        let points = lines
            .iter()
            .map(|line| {
                let pair: Vec<Point> = line
                    .split(" -> ")
                    .map(|x| {
                        let point: Vec<i32> = x
                            .split(',')
                            .map(|y| y.parse().expect("bad number"))
                            .collect();
                        Point {
                            x: point[0],
                            y: point[1],
                        }
                    })
                    .collect();
                (pair[0], pair[1])
            })
            .collect();

        Self { points }
    }
}

impl Day for Day5 {
    fn new<P: AsRef<Path>>(path: P) -> Self {
        let lines = util::read_lines(path).expect("bad input");
        Self::parse(&lines)
    }

    fn part1(&self) -> i32 {
        let points = self.points.iter().filter(|(a, b)| a.x == b.x || a.y == b.y);
        let mut map: HashMap<Point, i32> = HashMap::new();

        for (p1, p2) in points {
            if p1.x == p2.x {
                let range = if p2.y > p1.y {
                    p1.y..=p2.y
                } else {
                    p2.y..=p1.y
                };
                for y in range {
                    let p = Point { x: p1.x, y };
                    map.entry(p).and_modify(|acc| *acc += 1).or_insert(1);
                }
            } else {
                let range = if p2.x > p1.x {
                    p1.x..=p2.x
                } else {
                    p2.x..=p1.x
                };
                for x in range {
                    let p = Point { x, y: p1.y };
                    map.entry(p).and_modify(|acc| *acc += 1).or_insert(1);
                }
            }
        }

        let overlap = map.iter().map(|(_, v)| if *v > 1 { 1 } else { 0 }).sum();

        overlap
    }

    fn part2(&self) -> i32 {
        let mut map: HashMap<Point, i32> = HashMap::new();

        for (p1, p2) in self.points.iter() {
            if p1.x == p2.x {
                let range = if p2.y > p1.y {
                    p1.y..=p2.y
                } else {
                    p2.y..=p1.y
                };
                for y in range {
                    let p = Point { x: p1.x, y };
                    map.entry(p).and_modify(|acc| *acc += 1).or_insert(1);
                }
            } else if p1.y == p2.y {
                let range = if p2.x > p1.x {
                    p1.x..=p2.x
                } else {
                    p2.x..=p1.x
                };
                for x in range {
                    let p = Point { x, y: p1.y };
                    map.entry(p).and_modify(|acc| *acc += 1).or_insert(1);
                }
            } else {
                let (xd, diff) = if p1.x > p2.x {
                    (-1, p1.x - p2.x)
                } else {
                    (1, p2.x - p1.x)
                };
                let yd = if p1.y > p2.y { -1 } else { 1 };
                for i in 0..=diff {
                    let p = Point {
                        x: p1.x + i * xd,
                        y: p1.y + i * yd,
                    };
                    map.entry(p).and_modify(|acc| *acc += 1).or_insert(1);
                }
            }
        }

        let overlap = map.iter().map(|(_, v)| if *v > 1 { 1 } else { 0 }).sum();

        overlap
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_parse5() {
        let lines: Vec<String> = INPUT.split('\n').map(|s| s.to_owned()).collect();
        let x = Day5::parse(&lines);
        for pair in x.points {
            println!("{:?}", pair)
        }
    }

    #[test]
    fn part1() {
        let lines: Vec<String> = INPUT.split('\n').map(|s| s.to_owned()).collect();
        let x = Day5::parse(&lines);
        assert_eq!(5, x.part1());
    }

    #[test]
    fn part2() {
        let lines: Vec<String> = INPUT.split('\n').map(|s| s.to_owned()).collect();
        let x = Day5::parse(&lines);
        assert_eq!(12, x.part2());
    }
}
