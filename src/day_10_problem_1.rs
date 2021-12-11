use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn get_points_for_bracket(bracket: char) -> i32 {
    match bracket {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn get_closing_bracket(bracket: char) -> Option<char> {
    match bracket {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

fn is_opening_bracket(bracket: char) -> bool {
    match bracket {
        '(' => true,
        '[' => true,
        '{' => true,
        '<' => true,
        _ => false,
    }
}

pub fn day_10_problem_1() -> io::Result<i32> {
    let path_to_read = Path::new("./src/day-10-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let mut syntax_error_score = 0;
    for line in file_contents.lines() {
        let mut brackets = Vec::new();
        for bracket in line.chars() {
            if is_opening_bracket(bracket) {
                brackets.push(bracket);
            } else {
                let opening_bracket = brackets.pop().unwrap();
                if get_closing_bracket(opening_bracket).unwrap() != bracket {
                    syntax_error_score += get_points_for_bracket(bracket);
                    break;
                }
            }
        }
    }
    Ok(syntax_error_score)
}
