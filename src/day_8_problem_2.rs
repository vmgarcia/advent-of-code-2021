use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;

fn get_digit_from_segment_count(segment_count: usize) -> Option<u32> {
  match segment_count {
    2 => Some(1),
    3 => Some(7),
    4 => Some(4),
    7 => Some(8),
    _ => None
  }
}

fn convert_vec_to_int(ints: &Vec<u32>) -> u32 {
  let mut result = 0;
  for i in ints.iter() {
    result = result * 10 + i;
  }
  result
}

pub fn day_8_problem_2() -> io::Result<u32> {
    let path_to_read = Path::new("./src/day-8-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let mut result = Vec::new();
    for line in file_contents.lines() {
      let (front, end) = line.split_once(" | ").unwrap();
      let front_split = front.split_whitespace();
      let mut lookup = HashMap::new();
      for code in front_split.clone() {
        if let Some(digit) = get_digit_from_segment_count(code.len()) {
          lookup.insert(digit, code.chars().collect::<HashSet<char>>());
        }
      }
      
      let mut codes = Vec::new();
      let one = lookup.get(&1).unwrap().clone();
      let four = lookup.get(&4).unwrap().clone();
      let four_without_one: HashSet<char> = four.difference(&one).map(|x| *x).collect();
      // get code for 3 and 4
      front_split.for_each(|code| {
        let code_set = code.chars().collect::<HashSet<char>>();
        if code.len() == 5 && code_set.is_superset(&one) {
          lookup.insert(3, code_set);
        } else if code.len() == 5 && code_set.is_superset(&four_without_one) {
          lookup.insert(5, code_set);
        } else {
          codes.push(code_set);
        }
      });

      let five = lookup.get(&5).unwrap();
      let four = lookup.get(&4).unwrap();
      let nine: HashSet<char> = five.union(&four).map(|x| *x).collect();
      lookup.insert(9, nine.clone());
      codes = codes.into_iter().filter(|code| {
        if code.len() == 5 {
          lookup.insert(2, code.clone());
          return false;
        }
        true
      }).collect();

      let eight = lookup.get(&8).unwrap();
      let three = lookup.get(&3).unwrap();
      let four = lookup.get(&4).unwrap();
      let three_diff_four: HashSet<char> = three.difference(&four).map(|x| *x).collect();
      let eight_diff_three: HashSet<char> = eight.difference(&three).map(|x| *x).collect();
      let zero: HashSet<char> = one.union(&(eight_diff_three.union(&three_diff_four).map(|x| *x).collect())).map(|x| *x).collect();
      lookup.insert(0, zero.clone());
      codes.iter().for_each(|x| {
        if !lookup.values().any(|y| y == x) {
          lookup.insert(6, x.clone());
        }
      });
      let code_value: Vec<u32> = end.split_whitespace().map(|code| {
        let code_set = code.chars().collect::<HashSet<char>>();
        for (val, code_lookup) in &lookup {
          if *code_lookup == code_set {
            return *val;
          } 
        }

        0
      }).collect();
      result.push(convert_vec_to_int(&code_value));
    }

    Ok(result.iter().sum::<u32>())
}