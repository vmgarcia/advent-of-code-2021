use std::cmp;
use std::collections::{HashMap};
use std::io;

fn play_player_2(
  lookup: &mut HashMap<(u64, u64, u64, u64, u64, u64), (u64, u64)>,
  player_1_position: u64,
  player_2_position: u64,
  player_1_score: u64,
  player_2_score: u64,
  player_2_dice_roll: u64 
) -> (u64, u64) {

  if let Some(res) = lookup.get(&(player_1_position, player_1_score, player_2_position, player_2_score, 0, player_2_dice_roll)) {
    return *res;
  }

  let next_player_2_position = (player_2_position + player_2_dice_roll) % 10;
  let next_player_2_score = next_player_2_position + 1 + player_2_score;

  if next_player_2_score > 20 { return (0, 1) }
  let mut p1_wins = 0;
  let mut p2_wins = 0;
  for (roll, freq) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]  {
    let (wins1, wins2) = play_player_1(lookup, player_1_position, next_player_2_position, player_1_score, next_player_2_score, roll);
    p1_wins += wins1 * freq;
    p2_wins += wins2 * freq;
  }
  lookup.insert((player_1_position, player_1_score, player_2_position, player_2_score, 0, player_2_dice_roll), (p1_wins, p2_wins));

  (p1_wins, p2_wins)
}
fn play_player_1(
  lookup: &mut HashMap<(u64, u64, u64, u64, u64, u64), (u64, u64)>,
  player_1_position: u64,
  player_2_position: u64,
  player_1_score: u64,
  player_2_score: u64,
  player_1_dice_roll: u64 
) -> (u64, u64) {

  if let Some(res) = lookup.get(&(player_1_position, player_1_score, player_2_position, player_2_score, player_1_dice_roll, 0)) {
    return *res;
  }

  let next_player_1_position = (player_1_position + player_1_dice_roll) % 10;
  let next_player_1_score = next_player_1_position + 1 + player_1_score;
  if next_player_1_score > 20 { return (1, 0) }
  let mut p1_wins = 0;
  let mut p2_wins = 0;
  for (roll, freq) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]  {
    let (wins1, wins2) = play_player_2(lookup, next_player_1_position, player_2_position, next_player_1_score, player_2_score, roll);
    p1_wins += wins1 * freq;
    p2_wins += wins2 * freq;
  }
  lookup.insert((player_1_position, player_1_score, player_2_position, player_2_score, player_1_dice_roll, 0), (p1_wins, p2_wins));
  (p1_wins, p2_wins)
}

fn quantum_game(
  lookup: &mut HashMap<(u64, u64, u64, u64, u64, u64), (u64, u64)>,
  player_1_position: u64,
  player_2_position: u64,
  player_1_score: u64,
  player_2_score: u64,
) -> (u64, u64) {
  let mut p1_wins = 0;
  let mut p2_wins = 0;
  for (roll, freq) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]  {
    let (wins1, wins2) = play_player_1(lookup, player_1_position, player_2_position, player_1_score, player_2_score, roll);
    p1_wins += wins1 * freq;
    p2_wins += wins2 * freq;
  }
  (p1_wins, p2_wins)
}

pub fn day_21_problem_2() -> io::Result<u64> {
    let player_1_position = 2;
    let player_2_position = 9;

    let player_1_score = 0;
    let player_2_score = 0;

    let mut lookup = HashMap::new();
    let (p1_wins, p2_wins) = quantum_game(&mut lookup, player_1_position, player_2_position, player_1_score, player_2_score);
    Ok(cmp::max(p1_wins, p2_wins))
}
