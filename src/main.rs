use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

mod day1;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let lines: Vec<String> = lines_from_file("input/day1.txt");
    println!("day 1 part 1: {:?}", day1::part1(&lines));
    println!("day 1 part 2: {:?}", day1::part2(&lines));
}
