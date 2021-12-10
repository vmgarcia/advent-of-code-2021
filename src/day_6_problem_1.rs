use std::cmp;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn parse_ints(line: &str) -> Vec<u32> {
    line.split(",")
        .map(|x| u32::from_str_radix(x, 10).unwrap())
        .collect()
}

pub fn day_6_problem_1() -> io::Result<usize> {
    let path_to_read = Path::new("./src/day-6-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let mut population = parse_ints(file_contents.as_str());

    for day in 0..80 {
        let mut new_fish_count = 0;
        population = population
            .iter()
            .map(|fish| {
                if *fish == 0 {
                    new_fish_count += 1;
                    6
                } else {
                    fish - 1
                }
            })
            .collect();
        for i in 0..new_fish_count {
            population.push(8);
        }
    }
    Ok(population.len())
}
