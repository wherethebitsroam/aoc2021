use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub fn read_input<P: AsRef<Path>>(path: P) -> Result<Vec<i32>, Box<dyn Error>> {
    let f = File::open(path)?;
    let r = BufReader::new(f);

    // let iter: Vec<Result<i32, Box<dyn Error>>> =
    //     r.lines().map(|l| l.map(|l| l.parse::<i32>())).collect();

    let mut v = Vec::new();
    for l in r.lines() {
        let i = l?.parse()?;
        v.push(i);
    }

    Ok(v)
}

pub fn read_lines<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Box<dyn Error>> {
    let f = File::open(path)?;
    let r = BufReader::new(f);

    // let iter: Vec<Result<i32, Box<dyn Error>>> =
    //     r.lines().map(|l| l.map(|l| l.parse::<i32>())).collect();

    let mut v = Vec::new();
    for l in r.lines() {
        v.push(l?);
    }

    Ok(v)
}
