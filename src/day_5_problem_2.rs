use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;
use std::cmp;

fn parse_pair(pair: &str) -> (u32, u32) {
  let (a, b) = pair.split_once(",").unwrap();
  (a.parse().unwrap(), b.parse().unwrap())
}

#[test]
fn test_parse_pair() {
  assert_eq!(parse_pair("0,0"), (0,0));
}

fn get_points_in_line(a: (u32, u32), b: (u32, u32)) -> Vec<(u32, u32)>{
  let mut result = Vec::new();
  if cmp::max(a.0, b.0) - cmp::min(a.0, b.0) == cmp::max(a.1, b.1) - cmp::min(a.1, b.1) {
    if a.0 < b.0 && a.1 < b.1 || a.0 > b.0 && a.1 > b.1 {
      let diff = cmp::max(a.0, b.0) - cmp::min(a.0, b.0);
      let min;
      if a.0 < b.0 {
        min = a;
      } else {
        min = b;
      }
      for i in 0..=diff {
        result.push((min.0+i, min.1+i));
      }  
    } else {
      let mut start;
      let end;
      if a.0 < b.0 {
        start = (a.0, a.1);
        end = (b.0, b.1);
      } else {
        start = (b.0, b.1);
        end = (a.0, a.1);
      }
  
      while start != end {
        result.push(start);
        start = (start.0 + 1, start.1 - 1);
      }
      result.push(start);  
    }
  } else if a.0 == b.0 {
    let start = cmp::min(a.1, b.1);
    let end = cmp::max(a.1, b.1);
    for i in start..=end {
      result.push((a.0, i));
    }
  } else if a.1 == b.1 {
    let start = cmp::min(a.0, b.0);
    let end = cmp::max(a.0, b.0);
    for i in start..=end {
      result.push((i, a.1));
    }
  }
  result
}

#[test]
fn test_get_points_in_line() {
  assert_eq!(
    get_points_in_line((0,0), (0, 5)),
    vec![(0,0), (0,1), (0,2), (0,3), (0,4), (0,5)]
  );

  assert_eq!(
    get_points_in_line((0,0), (3,0)),
    vec![(0,0), (1,0), (2,0), (3,0)]
  );
  
  assert_eq!(
    get_points_in_line((0,0), (5,5)),
    vec![(0,0), (1,1), (2,2), (3,3), (4,4), (5,5)]
  );
  assert_eq!(
    get_points_in_line((10,6), (6,10)),
    vec![(6,10), (7,9), (8,8), (9,7), (10,6)]
  );
}

pub fn day_5_problem_2() -> io::Result<usize> {
    let path_to_read = Path::new("./src/day-5-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let mut marked_points: HashSet<(u32, u32)> = HashSet::new();
    let mut overlapped_points: HashSet<(u32, u32)> = HashSet::new();
    for line in file_contents.lines() {
      if let Some((a, b)) = line.split_once(" -> ") {
        let points_in_line = get_points_in_line(parse_pair(a),
          parse_pair(b));
        for point in points_in_line {
          if !marked_points.insert(point) {
            overlapped_points.insert(point);
          }
        }
      }
    }
    Ok(overlapped_points.len())
}
