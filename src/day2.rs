use crate::util;
use crate::Day;
use std::ops::Add;
use std::path::Path;

enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Direction {
    fn from_line(line: &str) -> Self {
        let blah: Vec<&str> = line.split(' ').collect();
        let v: i32 = blah[1].parse().expect("i32");
        match blah[0] {
            "forward" => Direction::Forward(v),
            "down" => Direction::Down(v),
            "up" => Direction::Up(v),
            d => panic!("bad direction {}", d),
        }
    }
}

struct Pos {
    f: i32,
    d: i32,
}

impl Add<Direction> for Pos {
    type Output = Pos;
    fn add(self, dir: Direction) -> Pos {
        match dir {
            Direction::Forward(i) => Pos {
                d: self.d,
                f: self.f + i,
            },
            Direction::Down(i) => Pos {
                d: self.d + i,
                f: self.f,
            },
            Direction::Up(i) => Pos {
                d: self.d - i,
                f: self.f,
            },
        }
    }
}

struct Pos2 {
    f: i32,
    d: i32,
    aim: i32,
}

impl Add<Direction> for Pos2 {
    type Output = Pos2;
    fn add(self, dir: Direction) -> Pos2 {
        match dir {
            Direction::Forward(i) => Pos2 {
                d: self.d + self.aim * i,
                f: self.f + i,
                aim: self.aim,
            },
            Direction::Down(i) => Pos2 {
                d: self.d,
                f: self.f,
                aim: self.aim + i,
            },
            Direction::Up(i) => Pos2 {
                d: self.d,
                f: self.f,
                aim: self.aim - i,
            },
        }
    }
}

pub struct Day2 {
    lines: Vec<String>,
}

impl Day for Day2 {
    fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            lines: util::read_lines(path).expect("bad input"),
        }
    }

    fn part1(&self) {
        let mut pos = Pos { d: 0, f: 0 };
        for line in self.lines.iter() {
            let dir = Direction::from_line(line);
            pos = pos + dir;
        }
        println!("d: {}, f: {}, dxf: {}", pos.d, pos.f, pos.d * pos.f);
    }

    fn part2(&self) {
        let mut pos = Pos2 { d: 0, f: 0, aim: 0 };
        for line in self.lines.iter() {
            let dir = Direction::from_line(line);
            pos = pos + dir;
        }
        println!("d: {}, f: {}, dxf: {}", pos.d, pos.f, pos.d * pos.f);
    }
}
