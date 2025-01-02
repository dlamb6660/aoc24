use std::{
    fs,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

mod day1;
mod day2;
mod day3;
mod day4;

mod day24;

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

    let lines: Vec<String> = lines_from_file("input/day2.txt");
    println!("day 2 part 1: {:?}", day2::part1(&lines));
    println!("day 2 part 2: {:?}", day2::part2(&lines));

    let txt: String = fs::read_to_string("input/day3.txt").unwrap();
    println!("day 3 part 1: {:?}", day3::part1(&txt));
    println!("day 3 part 2: {:?}", day3::part2(&txt));

    let lines: Vec<String> = lines_from_file("input/day4.txt");
    println!("day 4 part 1: {:?}", day4::part1(&lines));
    println!("day 4 part 2: {:?}", day4::part2(&lines));

    let lines: Vec<String> = lines_from_file("input/day24.txt");
    println!("day 24 part 1: {:?}", day24::part1(&lines));
    // println!("day 24 part 2: {:?}", day24::part2(&lines));
}
