use regex::Regex;

pub fn part1(txt: &String) -> i64 {
    // match mul(x,y)
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;

    for captures in regex.captures_iter(txt) {
        let x: i64 = captures[1].parse().unwrap();
        let y: i64 = captures[2].parse().unwrap();
        total += x * y;
    }
    
    total
}

pub fn part2(txt: &String) -> i64 {
    // match mul(x,y) or don't() or do()
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();
    let mut total = 0;
    let mut do_mul = true;

    for captures in regex.captures_iter(txt) {
        if let Some(_) = captures.get(0) {
            // check if mul
            if let (Some(x), Some(y)) = (captures.get(1), captures.get(2)) {
                if do_mul {
                    let x: i64 = x.as_str().parse().unwrap();
                    let y: i64 = y.as_str().parse().unwrap();
                    total += x * y;
                }
            } else if captures.get(0).map_or(false, |m| m.as_str() == "do()") {
                do_mul = true;
            } else if captures.get(0).map_or(false, |m| m.as_str() == "don't()") {
                do_mul = false;
            }
        }
    }
    
    total
}
