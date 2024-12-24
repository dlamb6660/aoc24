use regex::Regex;
use std::collections::HashMap;

pub fn part1(lines: &Vec<String>) -> usize {
    let mut grid_map = HashMap::new();

    let rows = lines.clone();
    let reversed_rows: Vec<String> = rows
        .iter()
        .map(|s| s.chars().rev().collect::<String>())
        .collect();
    let mut columns: Vec<String> = Vec::new();
    let mut diag_top_left_to_bot_right: Vec<String> = Vec::new();
    let mut diag_top_left_to_bot_right_second_half: Vec<String> = Vec::new();
    let mut diag_bot_left_to_top_right: Vec<String> = Vec::new();
    let mut diag_bot_left_to_top_right_second_half: Vec<String> = Vec::new();

    let height = rows.len();
    let width = rows[0].len();

    for (j, row) in rows.iter().enumerate() {
        for (i, letter) in row.chars().enumerate() {
            grid_map.insert((i, j), letter);
        }
    }

    for index in 0..width {
        columns.push(String::new());
        diag_top_left_to_bot_right.push(String::new());
        diag_top_left_to_bot_right_second_half.push(String::new());
        diag_bot_left_to_top_right.push(String::new());
        diag_bot_left_to_top_right_second_half.push(String::new());

        for y in 0..height {
            columns[index].push(*grid_map.get(&(index, y)).unwrap());
        }

        for j in 0..height {
            let y_index = height - 1 - index + j;
            let x_index = j;
            if y_index >= height || x_index >= width {
                break;
            }
            diag_top_left_to_bot_right[index].push(*grid_map.get(&(x_index, y_index)).unwrap());
            diag_bot_left_to_top_right[index].push(*grid_map.get(&(width-1-x_index, y_index)).unwrap());

            if index < width - 1 {
                // for every case except the last (the middle line), also add the mirrored diagonal
                diag_top_left_to_bot_right_second_half[index].push(*grid_map.get(&(y_index, x_index)).unwrap());
                diag_bot_left_to_top_right_second_half[index].push(*grid_map.get(&(x_index, index-j)).unwrap());
            }
        }
    }

    let reversed_columns: Vec<String> = columns
        .iter()
        .map(|s| s.chars().rev().collect::<String>())
        .collect();
    let reversed_diag_top_left_to_bot_right: Vec<String> = diag_top_left_to_bot_right
        .iter()
        .map(|s| s.chars().rev().collect::<String>())
        .collect();
    let reversed_diag_top_left_to_bot_right_second_half: Vec<String> = diag_top_left_to_bot_right_second_half
        .iter()
        .map(|s| s.chars().rev().collect::<String>())
        .collect();
    let reversed_diag_bot_left_to_top_right: Vec<String> = diag_bot_left_to_top_right
        .iter()
        .map(|s| s.chars().rev().collect::<String>())
        .collect();
    let reversed_diag_bot_left_to_top_right_second_half: Vec<String> = diag_bot_left_to_top_right_second_half
        .iter()
        .map(|s| s.chars().rev().collect::<String>())
        .collect();

    let re = Regex::new(r"XMAS").unwrap();

    let mut combined: Vec<String> = Vec::new();
    combined.extend(rows);
    combined.extend(columns);
    combined.extend(diag_top_left_to_bot_right);
    combined.extend(diag_top_left_to_bot_right_second_half);
    combined.extend(diag_bot_left_to_top_right);
    combined.extend(diag_bot_left_to_top_right_second_half);
    combined.extend(reversed_rows);
    combined.extend(reversed_columns);
    combined.extend(reversed_diag_top_left_to_bot_right);
    combined.extend(reversed_diag_top_left_to_bot_right_second_half);
    combined.extend(reversed_diag_bot_left_to_top_right);
    combined.extend(reversed_diag_bot_left_to_top_right_second_half);
    
    let total_occurrences = combined.iter()
        .map(|s| re.find_iter(s).count())
        .sum::<usize>();
        
    total_occurrences
}

pub fn part2(lines: &Vec<String>) -> usize {
    let mut grid_map = HashMap::new();

    let rows = lines.clone();
    let height = rows.len();
    let width = rows[0].len();

    let mut crosses: Vec<String> = Vec::new();

    for (j, row) in rows.iter().enumerate() {
        for (i, letter) in row.chars().enumerate() {
            grid_map.insert((i, j), letter);
        }
    }

    let mut index = 0;

    for x in 1..width-1 {
        for y in 1..height-1 {
            crosses.push(String::new());
            crosses[index].push(*grid_map.get(&(x-1, y-1)).unwrap());
            crosses[index].push(*grid_map.get(&(x, y)).unwrap());
            crosses[index].push(*grid_map.get(&(x+1, y+1)).unwrap());
            crosses[index].push(*grid_map.get(&(x-1, y+1)).unwrap());
            crosses[index].push(*grid_map.get(&(x, y)).unwrap());
            crosses[index].push(*grid_map.get(&(x+1, y-1)).unwrap());
            index += 1;
        }
    }

    let patterns = vec![
        Regex::new(r"MASMAS").unwrap(),
        Regex::new(r"MASSAM").unwrap(),
        Regex::new(r"SAMSAM").unwrap(),
        Regex::new(r"SAMMAS").unwrap(),
    ];

    let total_occurrences = patterns.iter().map(|re| {
        crosses.iter().map(|s| re.find_iter(s).count()).sum::<usize>()
    }).sum();

    total_occurrences
}
