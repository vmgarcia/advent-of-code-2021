use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn parse_input(input: String) -> (Vec<Vec<u32>>, (usize, usize)) {
    let mut parsed_input = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for ch in line.chars() {
            row.push(ch.to_digit(10).unwrap());
        }
        parsed_input.push(row);
    }

    let rows = parsed_input.len();
    let cols = parsed_input[0].len();
    (parsed_input, (rows, cols))
}

fn get_neighbors(
    (row, col): (usize, usize),
    (rows, cols): (usize, usize),
) -> [Option<(usize, usize)>; 4] {
    let mut neighbors = [None; 4];
    let transformations = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    for (i, (dr, dc)) in transformations.iter().enumerate() {
        let (rp, cp) = (dr + row as i32, dc + col as i32);
        if rp >= 0 && rp < rows as i32 && cp >= 0 && cp < cols as i32 {
            neighbors[i] = Some((rp as usize, cp as usize));
        } else {
            neighbors[i] = None;
        }
    }
    neighbors
}

pub fn day_15_problem_1() -> io::Result<u32> {
    let path_to_read = Path::new("./src/day-15-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let (map, (rows, cols)) = parse_input(file_contents);
    let destination = (rows - 1, cols - 1);

    let mut heap: BinaryHeap<(Reverse<u32>, u32, (usize, usize))> = BinaryHeap::new();
    heap.push((Reverse(0), 0, (0, 0)));
    let mut path_costs: HashMap<(usize, usize), u32> = HashMap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while heap.len() > 0 {
        let (_, weight, current) = heap.pop().unwrap();
        if visited.contains(&current) {
            continue;
        }
        if current == destination {
            return Ok(weight);
        }

        visited.insert(current);
        for n in get_neighbors(current, (rows, cols)) {
            if let Some((row, col)) = n {
                let current_weight = map[row][col] + weight;
                if let Some(previous_weight) = path_costs.get(&(row, col)) {
                    if current_weight < *previous_weight {
                        path_costs.insert((row, col), current_weight);
                        heap.push((Reverse(current_weight), current_weight, (row, col)));
                    }
                } else {
                    path_costs.insert((row, col), current_weight);
                    heap.push((Reverse(current_weight), current_weight, (row, col)));
                }
            }
        }
    }
    Ok(*path_costs.get(&destination).unwrap())
}
