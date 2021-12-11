use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn parse_input(input: String) -> [u8; 100] {
    let mut parsed_input = [0u8; 100];
    let mut index = 0;
    for line in input.lines() {
        for ch in line.chars() {
            parsed_input[index] = ch.to_digit(10).unwrap() as u8;
            index += 1;
        }
    }

    parsed_input
}

fn convert_index_to_point(index: usize) -> (i32, i32) {
    (index as i32 / 10, index as i32 % 10)
}

fn convert_point_to_index((row, col): (i32, i32)) -> usize {
    (row * 10 + col) as usize
}

fn get_neighbors_as_points((row, col): (i32, i32)) -> [Option<(i32, i32)>; 8] {
    let mut neighbors = [None; 8];
    let transformations = [
        (-1, -1),
        (-1, 0),
        (0, -1),
        (1, 1),
        (1, 0),
        (0, 1),
        (-1, 1),
        (1, -1),
    ];
    for (i, (dr, dc)) in transformations.iter().enumerate() {
        let (rp, cp) = (dr + row, dc + col);
        if rp >= 0 && rp < 10 && cp >= 0 && cp < 10 {
            neighbors[i] = Some((rp, cp));
        } else {
            neighbors[i] = None;
        }
    }
    neighbors
}

fn get_neighbors(index: usize) -> [Option<usize>; 8] {
    let index_as_point = convert_index_to_point(index);
    get_neighbors_as_points(index_as_point).map(|opt_neighbor| {
        if let Some(neighbor) = opt_neighbor {
            Some(convert_point_to_index(neighbor))
        } else {
            None
        }
    })
}

pub fn day_11_problem_1() -> io::Result<u64> {
    let path_to_read = Path::new("./src/day-11-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let mut octo_grid = parse_input(file_contents);

    let iterations = 100;
    let mut total_flashes = 0;
    for _ in 0..iterations {
        let mut flashing_octopi = Vec::new();
        for i in 0..octo_grid.len() {
            let next_energy_level = octo_grid[i] + 1;
            if next_energy_level > 9 {
                flashing_octopi.push(i);
            }
            octo_grid[i] = next_energy_level;
        }

        while flashing_octopi.len() > 0 {
            if let Some(flash_index) = flashing_octopi.pop() {
                get_neighbors(flash_index).into_iter().for_each(|n| {
                    if let Some(neighbor_index) = n {
                        let neighbor_energy_level = octo_grid[neighbor_index] + 1;
                        if neighbor_energy_level == 10 {
                            flashing_octopi.push(neighbor_index);
                        }
                        octo_grid[neighbor_index] = neighbor_energy_level;
                    }
                });
            }
        }

        octo_grid = octo_grid.map(|energy_level| {
            if energy_level > 9 {
                total_flashes += 1;
                0
            } else {
                energy_level
            }
        });
    }
    Ok(total_flashes)
}
