use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SnailFishNumber {
    Pair(Vec<SnailFishNumber>),
    Literal(i64),
}

fn split_at_outermost_comma(mut input: VecDeque<char>) -> (VecDeque<char>, VecDeque<char>) {
    let mut left_bracket_count = 0;
    for (i, ch) in input.iter().enumerate() {
        match ch {
            '[' => {
                left_bracket_count += 1;
            }
            ']' => {
                left_bracket_count -= 1;
            }
            ',' => {
                if left_bracket_count == 0 {
                    let mut right_hand = input.split_off(i);
                    right_hand.pop_front();
                    return (input, right_hand);
                }
            }
            _ => {
                continue;
            }
        }
    }
    unreachable!()
}

#[test]
fn test_split_at_outermost_comma() {
    let mut input = "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]"
        .chars()
        .collect::<VecDeque<char>>();
    input.pop_front();
    input.pop_back();
    let split = split_at_outermost_comma(input);
    assert_eq!(
        split,
        (
            "[[[1,3],[5,3]],[[1,3],[8,7]]]"
                .chars()
                .collect::<VecDeque<char>>(),
            "[[[4,9],[6,9]],[[8,2],[7,3]]]"
                .chars()
                .collect::<VecDeque<char>>()
        )
    );
}

fn parse_input(input: String) -> SnailFishNumber {
    fn parse_input_helper(mut input: VecDeque<char>) -> SnailFishNumber {
        let first_ch = input.pop_front().unwrap();
        match first_ch {
            '[' => {
                input.pop_back();
                let (left_chars, right_chars) = split_at_outermost_comma(input);
                let left = parse_input_helper(left_chars);
                let right = parse_input_helper(right_chars);
                SnailFishNumber::Pair(vec![left, right])
            }
            '0' => SnailFishNumber::Literal(0),
            '1' => SnailFishNumber::Literal(1),
            '2' => SnailFishNumber::Literal(2),
            '3' => SnailFishNumber::Literal(3),
            '4' => SnailFishNumber::Literal(4),
            '5' => SnailFishNumber::Literal(5),
            '6' => SnailFishNumber::Literal(6),
            '7' => SnailFishNumber::Literal(7),
            '8' => SnailFishNumber::Literal(8),
            '9' => SnailFishNumber::Literal(9),
            _ => unreachable!(),
        }
    }
    let snailfish_number = parse_input_helper(input.chars().collect::<VecDeque<char>>());
    snailfish_number
}

#[test]
fn test_parse_input() {
    let test_input = String::from("[[1,2],3]");
    let parsed_input = parse_input(test_input);
    assert_eq!(
        parsed_input,
        SnailFishNumber::Pair(vec![
            SnailFishNumber::Pair(vec![
                SnailFishNumber::Literal(1),
                SnailFishNumber::Literal(2)
            ]),
            SnailFishNumber::Literal(3)
        ])
    );
}

fn flatten(snailfish_num: &SnailFishNumber) -> VecDeque<(i64, i64)> {
    fn flatten_helper(snailfish_num: &SnailFishNumber, depth: i64) -> VecDeque<(i64, i64)> {
        let mut flat_list = VecDeque::new();
        match snailfish_num {
            SnailFishNumber::Pair(nested) => {
                if let (Some(left), Some(right)) = (nested.get(0), nested.get(1)) {
                    flat_list.append(&mut flatten_helper(left, depth + 1));
                    flat_list.append(&mut flatten_helper(right, depth + 1));
                }
            }
            SnailFishNumber::Literal(num) => {
                flat_list.push_back((depth, *num));
            }
        }
        flat_list
    }

    flatten_helper(snailfish_num, 0)
}

#[test]
fn test_flatten() {
    let test_input = String::from("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
    let parsed_input = parse_input(test_input);
    let flattened_input = flatten(&parsed_input);
    println!("{:?}", flattened_input)
    // assert_eq!(flattened_input, vec![1, 3, 5, 3, 1, 3, 8, 7, 4, 9, 6, 9, 8, 2, 7, 3]);
}

fn explode(mut list: VecDeque<(i64, i64)>) -> (VecDeque<(i64, i64)>, bool) {
    let mut new_vec = VecDeque::new();
    while list.len() > 0 {
        let (depth, val) = list.pop_front().unwrap();
        if depth == 5 {
            if let Some((p_depth, p_val)) = new_vec.pop_back() {
                new_vec.push_back((p_depth, p_val + val));
            }
            new_vec.push_back((4, 0));
            if let Some((_, r_val)) = list.pop_front() {
                if let Some((n_depth, n_val)) = list.pop_front() {
                    new_vec.push_back((n_depth, r_val + n_val));
                }
            }
            new_vec.append(&mut list);
            return (new_vec, true);
        } else {
            new_vec.push_back((depth, val));
        }
    }
    (new_vec, false)
}

#[test]
fn test_explode() {
    let input = String::from("[[[[[9,8],1],2],3],4]");
    let parsed_input = parse_input(input);
    let flattened_input = flatten(&parsed_input);
    let (exploded, did_explode) = explode(flattened_input);

    assert_eq!(
        exploded,
        VecDeque::from([(4, 0), (4, 9), (3, 2), (2, 3), (1, 4)])
    );
    assert!(did_explode);
}

#[test]
fn test_explode_2() {
    let input = String::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
    let parsed_input = parse_input(input);
    let flattened_input = flatten(&parsed_input);
    let (exploded, did_explode) = explode(flattened_input);
    assert_eq!(
        exploded,
        VecDeque::from([
            (2, 3),
            (3, 2),
            (4, 8),
            (4, 0),
            (2, 9),
            (3, 5),
            (4, 4),
            (5, 3),
            (5, 2)
        ])
    );
    assert!(did_explode);
    let (exploded_2, did_explode_2) = explode(exploded);
    assert_eq!(
        exploded_2,
        VecDeque::from([
            (2, 3),
            (3, 2),
            (4, 8),
            (4, 0),
            (2, 9),
            (3, 5),
            (4, 7),
            (4, 0)
        ])
    );
    assert!(did_explode_2);
}

fn split(mut list: VecDeque<(i64, i64)>) -> (VecDeque<(i64, i64)>, bool) {
    let mut new_vec = VecDeque::new();
    while list.len() > 0 {
        let (depth, val) = list.pop_front().unwrap();
        if val >= 10 {
            let left_val = val / 2;
            new_vec.push_back((depth + 1, left_val));
            let right_val = val - left_val;
            new_vec.push_back((depth + 1, right_val));

            new_vec.append(&mut list);
            return (new_vec, true);
        } else {
            new_vec.push_back((depth, val));
        }
    }
    (new_vec, false)
}

#[test]
fn test_split() {
    let flattened_input = VecDeque::from([
        (4, 0),
        (4, 7),
        (3, 4),
        (3, 15),
        (4, 0),
        (4, 13),
        (2, 1),
        (2, 1),
    ]);
    let (new_list, _) = split(flattened_input);
    let (new_list_2, did_split_2) = split(new_list);
    assert_eq!(
        new_list_2,
        VecDeque::from([
            (4, 0),
            (4, 7),
            (3, 4),
            (4, 7),
            (4, 8),
            (4, 0),
            (5, 6),
            (5, 7),
            (2, 1),
            (2, 1)
        ])
    );
    assert!(did_split_2);
}

fn reduce(list: VecDeque<(i64, i64)>) -> (VecDeque<(i64, i64)>, bool) {
    let (exploded_list, did_explode) = explode(list);
    let (split_list, did_split) = if !did_explode {
        split(exploded_list)
    } else {
        (exploded_list, false)
    };
    (split_list, did_split || did_explode)
}

fn add(mut list1: VecDeque<(i64, i64)>, mut list2: VecDeque<(i64, i64)>) -> VecDeque<(i64, i64)> {
    list1.append(&mut list2);
    let mut combined_list = list1
        .into_iter()
        .map(|(depth, val)| (depth + 1, val))
        .collect::<VecDeque<(i64, i64)>>();

    let mut should_reduce = true;
    while should_reduce {
        let (new_list, did_reduce) = reduce(combined_list);
        combined_list = new_list;
        should_reduce = did_reduce;
    }
    combined_list
}

#[test]
fn test_addition() {
    let input = String::from("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let parsed_input = parse_input(input);
    let input_2 = String::from("[1,1]");
    let parsed_input_2 = parse_input(input_2);
    let flattened_input = flatten(&parsed_input);
    let flattened_input_2 = flatten(&parsed_input_2);

    let summed = add(flattened_input, flattened_input_2);
    println!("{:?}", summed);
}

fn magnitude(v: VecDeque<(i64, i64)>) -> i64 {
    let mut vals = v.iter().map(|(_, vals)| *vals).collect::<Vec<i64>>();
    let mut depths = v.iter().map(|(depth, _)| *depth).collect::<Vec<i64>>();

    while vals.len() > 1 {
        for i in 0..depths.len() - 1 {
            if depths[i] == depths[i + 1] {
                vals[i] = 3 * vals[i] + 2 * vals[i + 1];
                vals.remove(i + 1);
                depths.remove(i + 1);

                if depths[i] > 0 {
                    depths[i] -= 1;
                }

                break;
            }
        }
    }
    vals[0]
} // #[test]
  // fn test_magnitude() {
  //   let mut list = VecDeque::from([(4, 6), (4, 6), (4, 7), (4, 8), (4, 7), (4, 7), (4, 8), (4, 7), (4, 9), (4, 9), (4, 8), (4, 0), (4, 9), (4, 8), (4, 8), (4, 7)]);
  //   while list.len() != 1 {
  //     list = magnitude(list);
  //   }
  //   println!("{:?}", list);

// }

pub fn day_18_problem_2() -> io::Result<i64> {
    let path_to_read = Path::new("./src/day-18-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    //     file_contents = String::from(
    //         "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    // [[[5,[2,8]],4],[5,[[9,9],0]]]
    // [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    // [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    // [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    // [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    // [[[[5,4],[7,7]],8],[[8,3],8]]
    // [[9,3],[[9,9],[6,[4,9]]]]
    // [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    // [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
    //     );

    let mut lines = file_contents.lines();
    let numbers = lines
        .map(|line| flatten(&parse_input(String::from(line))))
        .collect::<VecDeque<VecDeque<(i64, i64)>>>();
    let mut mags: Vec<i64> = Vec::new();
    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            if let (Some(n1), Some(n2)) = (numbers.get(i), numbers.get(j)) {
                let mut num1 = n1.clone();
                let mut num2 = n2.clone();
                mags.push(magnitude(add(num1.clone(), num2.clone())));
                mags.push(magnitude(add(num2, num1)));
            }
        }
    }
    Ok(mags.into_iter().max().unwrap())
}
