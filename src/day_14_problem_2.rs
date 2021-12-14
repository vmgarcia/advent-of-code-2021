use counter::Counter;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn parse_input(
    input: String,
) -> (
    Counter<(char, char), u64>,
    HashMap<(char, char), char>,
    char,
) {
    let mut insertion_rules = HashMap::new();
    let rule_regex = Regex::new(r"([A-Z])([A-Z]) -> ([A-Z])").unwrap();

    let mut input_lines = input.lines();
    let mut chars = input_lines.next().unwrap().chars().collect::<Vec<char>>();
    let pair_counts = Counter::from_iter(
        chars
            .iter()
            .as_slice()
            .windows(2)
            .map(|pair| (*pair.get(0).unwrap(), *pair.get(1).unwrap())),
    );
    let last_char = chars.pop().unwrap();
    for line in input_lines {
        if let Some(captures) = rule_regex.captures(line) {
            if let (Some(a), Some(b), Some(res)) =
                (captures.get(1), captures.get(2), captures.get(3))
            {
                insertion_rules.insert(
                    (
                        a.as_str().chars().next().unwrap(),
                        b.as_str().chars().next().unwrap(),
                    ),
                    res.as_str().chars().next().unwrap(),
                );
            }
        }
    }
    (pair_counts, insertion_rules, last_char)
}

pub fn day_14_problem_2() -> io::Result<u64> {
    let path_to_read = Path::new("./src/day-14-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let (mut pair_counts, insertion_rules, last_char) = parse_input(file_contents);
    for _ in 0..40 {
        let mut new_count = Counter::new();
        for ((a, b), count) in pair_counts.iter() {
            let result = insertion_rules.get(&(*a, *b)).unwrap();
            let new_a_count = *new_count.get(&(*a, *result)).unwrap_or(&0) + *count;
            let new_b_count = *new_count.get(&(*result, *b)).unwrap_or(&0) + *count;
            new_count.insert((*a, *result), new_a_count);
            new_count.insert((*result, *b), new_b_count);
        }
        pair_counts = new_count;
    }

    let mut char_counts = Counter::new();
    for ((a, b), count) in pair_counts.into_iter() {
        let new_a = char_counts.get(a).unwrap_or(&0) + count;
        char_counts.insert(*a, new_a);
    }
    let new_val = *char_counts.get(&last_char).unwrap() + 1;
    char_counts.insert(last_char, new_val);
    let mut ordered = char_counts.most_common();
    let (_, least_common) = ordered.pop().unwrap();
    let (_, most_common) = ordered.get(0).unwrap();
    Ok(most_common - least_common)
}
