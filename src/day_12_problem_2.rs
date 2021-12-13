use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

fn insert_or_default(map: &mut HashMap<String, HashSet<String>>, key: String, value: String) {
    if let Some(set) = map.get_mut(&key) {
        set.insert(value);
    } else {
        let set = HashSet::from([value]);
        map.insert(key, set);
    }
}

fn parse_input(input: String) -> HashMap<String, HashSet<String>> {
    let mut parsed_input = HashMap::new();
    for line in input.lines() {
        if let Some((a, b)) = line.split_once("-") {
            let a = String::from(a);
            let b = String::from(b);
            if b != String::from("start") {
                insert_or_default(&mut parsed_input, a.clone(), b.clone())
            };
            if a != String::from("start") {
                insert_or_default(&mut parsed_input, b, a)
            };
        }
    }
    parsed_input
}

pub fn day_12_problem_2() -> io::Result<i32> {
    let path_to_read = Path::new("./src/day-12-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let map = parse_input(file_contents);
    let mut paths = 0;
    let mut path = Vec::new();
    let mut visits = HashMap::new();
    let mut flag = true;
    fn find_paths(
        current: &String,
        path: &mut Vec<String>,
        paths: &mut i32,
        map: &HashMap<String, HashSet<String>>,
        visits: &mut HashMap<String, i32>,
        flag: &mut bool,
    ) {
        path.push(current.clone());
        visits.insert(current.clone(), visits.get(current).unwrap_or(&0) + 1);
        let mut set_flag = false;
        if current.to_lowercase() == *current && *flag && *visits.get(current).unwrap_or(&0) >= 2 {
            *flag = false;
            set_flag = true;
        }
        if let Some(neighbors) = map.get(current) {
            for neighbor in neighbors {
                if *neighbor == String::from("end") {
                    *paths += 1
                } else if neighbor.to_lowercase() != *neighbor
                    || *flag && *visits.get(neighbor).unwrap_or(&0) < 2
                    || *visits.get(neighbor).unwrap_or(&0) < 1
                {
                    find_paths(neighbor, path, paths, map, visits, flag);
                }
            }
        }
        path.pop();
        visits.insert(current.clone(), visits.get(current).unwrap_or(&0) - 1);
        if set_flag {
            *flag = true;
        }
    }
    find_paths(
        &String::from("start"),
        &mut path,
        &mut paths,
        &map,
        &mut visits,
        &mut flag,
    );
    Ok(paths)
}
