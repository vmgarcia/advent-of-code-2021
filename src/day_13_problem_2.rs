use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn parse_input(input: String) -> (HashSet<(u32, u32)>, Vec<(String, u32)>) {
    let mut points = HashSet::new();
    let mut folds = Vec::new();
    let point_regex = Regex::new(r"([0-9]+),([0-9]+)").unwrap();
    let fold_regex = Regex::new(r"fold along (x|y)=([0-9]+)").unwrap();
    for line in input.lines() {
        if let Some(p) = point_regex.captures(line) {
            if let (Some(left), Some(right)) = (p.get(1), p.get(2)) {
                points.insert((
                    left.as_str().parse().unwrap(),
                    right.as_str().parse().unwrap(),
                ));
            }
        } else if let Some(f) = fold_regex.captures(line) {
            if let (Some(direction), Some(line_no)) = (f.get(1), f.get(2)) {
                folds.push((
                    String::from(direction.as_str()),
                    line_no.as_str().parse().unwrap(),
                ));
            }
        }
    }
    (points, folds)
}

fn print_points(points: &HashSet<(u32, u32)>) {
    let max_x = points.iter().map(|(x, _)| *x).max().unwrap() as usize;
    let max_y = points.iter().map(|(_, y)| *y).max().unwrap() as usize;
    let mut bounding_array = vec![vec!['.'; max_x + 1]; max_y + 1];
    for (x, y) in points {
        bounding_array[*y as usize][*x as usize] = '0';
    }
    for row in bounding_array {
        for val in row {
            print!("{}", val);
        }
        print!("\n");
    }
}

pub fn day_13_problem_2() -> io::Result<usize> {
    let path_to_read = Path::new("./src/day-13-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let (mut points, folds) = parse_input(file_contents);
    for (direction, line) in folds {
        let mut next_points = HashSet::new();
        for (x, y) in points {
            if direction.as_str() == "x" && x > line {
                let new_x = line - (x - line);
                next_points.insert((new_x, y));
            } else if direction.as_str() == "y" && y > line {
                let new_y = line - (y - line);
                next_points.insert((x, new_y));
            } else {
                next_points.insert((x, y));
            }
        }
        points = next_points;
    }
    print_points(&points);
    Ok(points.len())
}
