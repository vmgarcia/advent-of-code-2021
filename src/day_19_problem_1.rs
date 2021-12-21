use regex::Regex;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn parse_input(input: String) -> VecDeque<Vec<(i64, i64, i64)>> {
    let header_regex = Regex::new(r"--- scanner ([0-9]+) ---").unwrap();
    let coordinate_regex = Regex::new(r"(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)").unwrap();

    let mut coordinates = Vec::new();
    let mut scanners = VecDeque::new();
    for line in input.lines() {
        if let Some(captures) = coordinate_regex.captures(line) {
            if let (Some(x), Some(y), Some(z)) = (captures.get(1), captures.get(2), captures.get(3))
            {
                coordinates.push((
                    x.as_str().parse().unwrap(),
                    y.as_str().parse().unwrap(),
                    z.as_str().parse().unwrap(),
                ));
            }
        } else if let Some(_) = header_regex.captures(line) {
            if coordinates.len() > 1 {
                scanners.push_back(coordinates);
            }
            coordinates = Vec::new();
        }
    }
    scanners.push_back(coordinates);
    scanners
}

pub fn day_19_problem_1() -> io::Result<i64> {
    let path_to_read = Path::new("./src/day-18-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    file_contents = String::from(
        "--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7

--- scanner 0 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0",
    );

    let input = parse_input(file_contents);

    Ok(0)
}
