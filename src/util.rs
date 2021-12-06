pub fn read_input(s: &str) -> Vec<i32> {
    let mut v = Vec::new();
    for l in s.split('\n') {
        let i = l.parse().expect("not a number");
        v.push(i);
    }

    v
}

pub fn read_lines(s: &str) -> Vec<String> {
    let mut v = Vec::new();
    for l in s.split('\n') {
        v.push(l.to_owned());
    }

    v
}
