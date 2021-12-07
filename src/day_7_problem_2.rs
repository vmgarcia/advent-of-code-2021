use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::cmp;
use rayon::prelude::*;

fn parse_ints(line: &str) -> Vec<u64> {
  line.split(",").map(|x| {
    u64::from_str_radix(x, 10).unwrap()
  }).collect()
}



pub fn day_7_problem_2() -> io::Result<u64> {
    let path_to_read = Path::new("./src/day-7-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    let mut crab_positions = parse_ints(&file_contents);
    crab_positions.sort();
    let max_position = crab_positions[crab_positions.len()-1];
    let fuel_needed = (0..=max_position).into_par_iter().map(|end_pos| {
      let mut total_fuel_needed = 0;
      for start_pos in &crab_positions {
        let current_fuel_needed = cmp::max(end_pos, *start_pos) - cmp::min(end_pos, *start_pos);
        for i in 1..=current_fuel_needed {
          total_fuel_needed += i;
        }
      }
      total_fuel_needed
    }).collect::<Vec<u64>>();
    Ok(*fuel_needed.iter().min().unwrap())
}