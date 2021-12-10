use std::cmp;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn parse_ints(line: &str) -> Vec<u64> {
    line.split(",")
        .map(|x| u64::from_str_radix(x, 10).unwrap())
        .collect()
}

pub fn day_7_problem_1() -> io::Result<u64> {
    let path_to_read = Path::new("./src/day-7-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    let mut crab_positions = parse_ints(&file_contents);
    crab_positions.sort();

    let fuel_needed = crab_positions
        .clone()
        .into_iter()
        .map(|end_pos| {
            let mut total_fuel_needed = 0;
            for start_pos in &crab_positions {
                total_fuel_needed += cmp::max(end_pos, *start_pos) - cmp::min(end_pos, *start_pos);
            }
            total_fuel_needed
        })
        .collect::<Vec<u64>>();
    Ok(*fuel_needed.iter().min().unwrap())
}
