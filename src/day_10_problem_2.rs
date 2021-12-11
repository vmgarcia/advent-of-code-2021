use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn get_points_for_bracket(bracket: char) -> u64 {
    match bracket {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0
    }
}

fn get_closing_bracket(bracket: char) -> Option<char> {
    match bracket {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None
    }
}

fn is_opening_bracket(bracket: char) -> bool {
    match bracket {
        '(' => true,
        '[' => true,
        '{' => true,
        '<' => true,
        _ => false
    }
}

pub fn day_10_problem_2() -> io::Result<u64> {
    let path_to_read = Path::new("./src/day-10-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let filtered_lines = file_contents.lines().filter(|line| {
      let mut brackets = Vec::new();
      for bracket in line.chars() {
          if is_opening_bracket(bracket) {
              brackets.push(bracket);
          } else {
              let opening_bracket = brackets.pop().unwrap();
              if get_closing_bracket(opening_bracket).unwrap() != bracket {
                  return false;
              } 
          }
      }
      true
    }).collect::<Vec<&str>>();

    let mut scores = filtered_lines.into_iter().map(|line| {
      let mut brackets = Vec::new();
      let mut score = 0;
      for bracket in line.chars() {
          if is_opening_bracket(bracket) {
              brackets.push(bracket);
          } else {
              let opening_bracket = brackets.pop().unwrap();
          }
      }
      while brackets.len() > 0 {
        let bracket = brackets.pop().unwrap();
        score = 5 * score + get_points_for_bracket(bracket);
      }
      score
    }).collect::<Vec<u64>>();
    scores.sort();
    Ok(scores[scores.len()/2])
}
