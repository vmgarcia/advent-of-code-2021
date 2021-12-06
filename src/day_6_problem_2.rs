use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn parse_ints(line: &str) -> Vec<(u64, u64)> {
  line.split(",").map(|x| {
    (1, u64::from_str_radix(x, 10).unwrap())
  }).collect()
}

pub fn day_6_problem_2() -> io::Result<u64> {
    let path_to_read = Path::new("./src/day-6-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    let mut population = parse_ints(file_contents.as_str());
  
    for _ in 0..256 {
      let mut new_fish_count = 0;
      population = population.iter().map(|(count, fish)| {
        if *fish == 0 {
          new_fish_count += count;
          (*count, 6)
        } else {
          (*count, fish - 1)
        }
      }).collect();
      population.push((new_fish_count, 8));
    }
    Ok(population.iter().fold(0, |size_acc, (size, _)| { size_acc + size }))
}