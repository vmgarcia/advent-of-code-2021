use rayon::prelude::*;
use regex::Regex;
use std::cmp;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

struct DeterministicDice {
    next_roll: u64,
    roll_count: u64,
}

impl DeterministicDice {
    fn new() -> Self {
        Self {
            next_roll: 1,
            roll_count: 0,
        }
    }

    fn roll_dice(&mut self) -> u64 {
        self.roll_count += 3;
        let roll_1 = self.next_roll;
        if self.next_roll + 1 > 100 {
            self.next_roll = 1;
        } else {
            self.next_roll += 1;
        }
        let roll_2 = self.next_roll;
        if self.next_roll + 1 > 100 {
            self.next_roll = 1;
        } else {
            self.next_roll += 1;
        }
        let roll_3 = self.next_roll;
        if self.next_roll + 1 > 100 {
            self.next_roll = 1;
        } else {
            self.next_roll += 1;
        }
        roll_1 + roll_2 + roll_3
    }
}

pub fn day_21_problem_1() -> io::Result<u64> {
    let mut player_1_position = 2;
    let mut player_2_position = 9;

    let mut player_1_score = 0;
    let mut player_2_score = 0;

    let mut dice = DeterministicDice::new();
    while player_1_score <= 1000 && player_2_score <= 1000 {
        player_1_position = (dice.roll_dice() + player_1_position) % 10;
        player_1_score += player_1_position + 1;
        if player_1_score >= 1000 {
            break;
        }
        player_2_position = (dice.roll_dice() + player_2_position) % 10;
        player_2_score += player_2_position + 1;
    }
    println!(
        "{}, {}, {}",
        player_1_score, player_2_score, dice.roll_count
    );
    Ok(cmp::min(player_1_score, player_2_score) * dice.roll_count)
}
