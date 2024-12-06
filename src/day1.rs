use std::iter::zip;
use std::collections::HashMap;


pub fn part1(lines: &Vec<String>) -> i32 {
    let mut lhs: Vec<i32> = Vec::new();
    let mut rhs: Vec<i32> = Vec::new();
    
    for line in lines.iter() {
        let mut parts = line.split_whitespace();
        let first: i32 = parts.next().unwrap().parse().unwrap();
        let second: i32 = parts.next().unwrap().parse().unwrap();
        lhs.push(first);
        rhs.push(second);
    }
    
    lhs.sort();
    rhs.sort();

    let mut diff: i32 = 0;

    for (l, r) in zip(lhs, rhs){
        diff += (l-r).abs();
    }
    
    diff
}


pub fn part2(lines: &Vec<String>) -> i32 {
    let mut lhs: Vec<i32> = Vec::new();
    let mut rhs_count = HashMap::new();

    for line in lines.iter() {
        let mut parts = line.split_whitespace();
        let first: i32 = parts.next().unwrap().parse().unwrap();
        let second: i32 = parts.next().unwrap().parse().unwrap();
        lhs.push(first);
        let count = rhs_count.entry(second).or_insert(0);
        *count += 1;
    }

    let mut total = 0;
    for l in lhs.iter() {
        let count = rhs_count.get(l);
        match count {
            Some(val) => total += l * val,
            None => (),
        }
    }

    total
}