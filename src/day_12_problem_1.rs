use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

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
            insert_or_default(&mut parsed_input, a.clone(), b.clone());
            insert_or_default(&mut parsed_input, b, a);
        }
    }
    parsed_input
}

pub fn day_12_problem_1() -> io::Result<usize> {
    let path_to_read = Path::new("./src/day-12-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    //     file_contents = String::from("start-A
    // start-b
    // A-c
    // A-b
    // b-d
    // A-end
    // b-end");
    let map = parse_input(file_contents);

    let mut paths = 0;
    let mut path = Vec::new();

    fn find_paths(
        start: &String,
        path: &mut Vec<String>,
        paths: &mut i32,
        map: &HashMap<String, HashSet<String>>,
    ) {
        fn find_paths_helper(
            current: &String,
            prev: &String,
            path: &mut Vec<String>,
            paths: &mut i32,
            map: &HashMap<String, HashSet<String>>,
        ) {
            path.push(current.clone());
            if let Some(neighbors) = map.get(current) {
                for neighbor in neighbors {
                    if *neighbor == String::from("end") {
                        *paths += 1;
                    } else if !(neighbor.to_lowercase() == *neighbor && path.contains(neighbor)) {
                        find_paths_helper(neighbor, current, path, paths, map);
                    }
                }
            }
            path.pop();
        }
        find_paths_helper(start, &String::from(""), path, paths, map);
    }
    find_paths(&String::from("start"), &mut path, &mut paths, &map);
    Ok(paths as usize)
}
