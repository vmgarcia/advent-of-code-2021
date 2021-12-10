use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;

fn parse_ints(line: &str) -> Vec<i32> {
  line.chars().map(|x| {
    x.to_digit(10).unwrap() as i32
  }).collect()
}

fn get_neighbors(position: (usize, usize), grid_height: usize, grid_width: usize) -> Vec<(usize, usize)> {
  let mut neighbors = Vec::new();
  let transformations: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
  let (x, y) = position;
  for (dx, dy) in transformations {
    let new_x = x as i32 + dx;
    let new_y = y as i32 + dy;
    if new_x >= 0 && (new_x as usize) < grid_height && new_y >= 0 && (new_y as usize) < grid_width {
      neighbors.push((new_x as usize, new_y as usize));
    }
  }
  neighbors
}

pub fn day_9_problem_2() -> io::Result<usize> {
    let path_to_read = Path::new("./src/day-9-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let mut height = 0;
    let mut width = 0;

    let mut map = Vec::new();

    for line in file_contents.lines() {
      if width == 0 { width = line.len(); }
      height += 1;
      map.push(parse_ints(line));
    }

    let mut valleys = Vec::new();
    for (row, line) in map.iter().enumerate() {
      for (col, val) in line.iter().enumerate() {
        let is_valley = !get_neighbors((row, col), height, width).into_iter().any(|(r, c)| {
          let neighbor = map[r][c];
          neighbor <= *val
        });
        if is_valley { valleys.push((row, col)); }
      }
    }

    let mut basin_sizes = Vec::new();
    for valley in valleys {
      let mut stack = vec![(valley)];
      let mut visited: HashSet<(usize, usize)> = HashSet::new();

      while stack.len() > 0 {
        let current = stack.pop().unwrap();
        visited.insert(current);
        get_neighbors(current, height, width).into_iter().for_each(|(row, col)| {
          if !visited.contains(&(row, col)) && map[row][col] != 9 {
            stack.push((row, col));
          }
        });
      }
      basin_sizes.push(visited.len());
    }
    basin_sizes.sort_by(|a, b| b.cmp(a));

    Ok(basin_sizes.iter().copied().take(3).product::<usize>())
}