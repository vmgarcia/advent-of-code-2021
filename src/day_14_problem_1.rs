use counter::Counter;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn parse_input(input: String) -> (Vec<char>, HashMap<(char, char), char>) {
    let mut template;
    let mut insertion_rules = HashMap::new();
    let rule_regex = Regex::new(r"([A-Z])([A-Z]) -> ([A-Z])").unwrap();

    let mut input_lines = input.lines();
    template = input_lines.next().unwrap().chars().collect();

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
    (template, insertion_rules)
}

pub fn day_14_problem_1() -> io::Result<u32> {
    let path_to_read = Path::new("./src/day-14-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    file_contents = String::from(
        "NNCB

  CH -> B
  HH -> N
  CB -> H
  NH -> C
  HB -> C
  HC -> B
  HN -> C
  NN -> C
  BH -> H
  NC -> B
  NB -> B
  BN -> B
  BB -> N
  BC -> B
  CC -> N
  CN -> C",
    );
    let (mut template, insertion_rules) = parse_input(file_contents);

    for i in 0..10 {
        let mut next_template = Vec::with_capacity(template.len() * 2);
        for p in template.windows(2) {
            // println!("{:?}, {:?}", p, template);
            if let (Some(a), Some(b)) = (p.get(0), p.get(1)) {
                if let Some(c) = insertion_rules.get(&(*a, *b)) {
                    // println!("c:: {:?}", c);
                    next_template.push(*a);
                    next_template.push(*c);
                }
            }
        }
        next_template.push(*template.get(template.len() - 1).unwrap());
        template = next_template;
    }

    let element_counts: Counter<char, u32> = Counter::from_iter(template.into_iter());
    let mut ordered = element_counts.most_common();
    let (_, least_common) = ordered.pop().unwrap();
    let (_, most_common) = ordered.get(0).unwrap();
    Ok(most_common - least_common)
}
