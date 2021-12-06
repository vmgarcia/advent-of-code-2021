use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;


fn parse_ints(line: &str) -> Vec<i32> {
  line.split_whitespace().map(|x| {
    i32::from_str_radix(x, 10).unwrap()
  }).collect()
}

fn parse_bingo_board<'a, I>(input: &mut I) -> Option<Vec<Vec<i32>>>
  where 
    I: Iterator<Item = &'a str>
{
  let mut board = Vec::new();

  let mut line = input.next()?;
  while !line.trim().is_empty() {
    board.push(parse_ints(line));
    line = input.next()?;
  }
  Some(board)
}

fn mark_board(num: i32, board: &mut Vec<Vec<i32>>) {
  for i in 0..board.len() {
    let line = &mut board[i];
    for j in 0..line.len() {
      if line[j] == num {
        line[j] = -1;
      }
    }
  }
}

#[test]
fn test_mark_board() {
  let mut board = vec![
    vec![10, 1, 11],
    vec![99, 110, 45],
    vec![76, 23, 111]
  ];
  mark_board(110, &mut board);
  assert_eq!(board[1][1], -1);
}

fn did_win(board: & Vec<Vec<i32>>) -> bool {
  let mut flag = false;
  for line in board {
    if !line.iter().any(| x| *x != -1) {
      return true;
    }
  }

  for i in 0..board.len() {
    let mut inner_flag = true;
    for j in 0..board.len() {
      if board[j][i] != -1 {
        inner_flag = false;
      }
    }
    if inner_flag {
      return true;
    }
  }
  false
}

#[test]
fn test_did_win() {
  assert!(did_win(
    &vec![
      vec![-1, 0, 10],
      vec![-1, 100, 50],
      vec![-1, -1, 20]
    ]
  ));
}

#[test]
fn test_did_win_2() {
  assert!(did_win(
    &vec![
      vec![-1, 0, 10],
      vec![-1, -1, -1],
      vec![7, 90, 70]
    ]
  ));
}

#[test]
fn test_did_not_win() {
  assert_eq!(did_win(
    &vec![
      vec![0, 10, 5],
      vec![4, 44, 10],
      vec![56, 92, 100]
    ]), false);
}

fn sum_of_unmarked_numbers(board: &Vec<Vec<i32>>) -> i32 {
  let mut total = 0;
  for line in board {
    for num in line {
      if *num != -1 {
        total += num;
      }
    }
  }
  total
}

pub fn day_4_problem_2() -> io::Result<i32> {
    let path_to_read = Path::new("./src/day-4-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let mut split = file_contents.split("\n");
    let mut numbers: Vec<i32> = split.next().unwrap().split(",").map(|x| {
      i32::from_str_radix(x, 10).unwrap()
    }).collect();

    let mut bingo_boards: Vec<Vec<Vec<i32>>> = Vec::new();
    split.next();
    loop {
      if let Some(bingo_board) = parse_bingo_board(&mut split) {
        bingo_boards.push(bingo_board);
      } else {
        break;
      }
    }

    for num in numbers {
      let bingo_boards_len = bingo_boards.len();
      for board in &mut bingo_boards {
        mark_board(num, board);
        if bingo_boards_len == 1 && did_win(board) {
          let total = sum_of_unmarked_numbers(board);
          return Ok(num * total);
        }
      }
      bingo_boards = bingo_boards.into_iter().filter(|board| !did_win(board)).collect();
    }

    Ok(0)
}
