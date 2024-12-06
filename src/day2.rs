pub fn part1(lines: &Vec<String>) -> i32 {
    find_num_safe_levels(lines, false)
}

pub fn part2(lines: &Vec<String>) -> i32 {
    find_num_safe_levels(lines, true)
}


fn find_num_safe_levels(lines: &Vec<String>, bad_level_tolerance: bool) -> i32 {
    let mut total_safe = 0;
    for line in lines.iter() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok()) 
            .collect();

        if check_level_is_safe(&levels) {
            total_safe += 1;
        } else if bad_level_tolerance {
            // just check all options, removing one level...
            for (i, _) in levels.iter().enumerate() {
                let mut levels_clone = levels.clone();
                levels_clone.remove(i);
                if check_level_is_safe(&levels_clone) {
                    total_safe += 1;
                    break;
                }
            }
        }
    }
    total_safe
}


fn check_level_is_safe(levels: &Vec<i32>) -> bool {
    let mut prev_level: Option<i32> = None;
    let mut ascending: Option<bool> = None;
    let mut is_safe = true;
    for level in levels {
        if prev_level.is_some() {
            let diff = level - prev_level.unwrap();
            if diff.abs() > 3 {
                // diff too big, ignore
                is_safe = false;
                break;
            } else if diff == 0 {
                // no diff, ignore
                is_safe = false;
                break;
            }
            if ascending.is_some() {
                if ascending.unwrap() && diff < 0 {
                    // was ascending, now descending, ignore
                    is_safe = false;
                    break;
                } else if !ascending.unwrap() && diff > 0 {
                    // was descending, now ascending, ignore
                    is_safe = false;
                    break;
                }
            } else {
                if diff > 0 {
                    ascending = Some(true);
                } else if diff < 0 {
                    ascending = Some(false);
                }
            }                
        }

        prev_level = Some(*level);
    }
    is_safe
}