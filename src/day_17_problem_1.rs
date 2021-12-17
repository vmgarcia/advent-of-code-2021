use std::cmp;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io;

#[derive(Debug, Clone, Copy)]
struct Velocity(i64, i64);
#[derive(Debug, Clone, Copy)]
struct Position(i64, i64);

fn apply_drag(x_velocity: i64) -> i64 {
    if x_velocity > 0 {
        x_velocity - 1
    } else if x_velocity < 0 {
        x_velocity + 1
    } else {
        0
    }
}

fn apply_gravity(y_velocity: i64) -> i64 {
    y_velocity - 1
}

fn tick(position: Position, velocity: Velocity) -> (Position, Velocity) {
    let next_position = Position(position.0 + velocity.0, position.1 + velocity.1);
    let next_velocity = Velocity(apply_drag(velocity.0), apply_gravity(velocity.1));
    (next_position, next_velocity)
}

fn find_min_x_velocity(x_target_bounds: (i64, i64)) -> i64 {
    let mut starting_x_velocity = 1;
    loop {
        let mut x_position = 0;
        let mut current_x_velocity = starting_x_velocity;
        while current_x_velocity != 0 {
            x_position += current_x_velocity;
            current_x_velocity = apply_drag(current_x_velocity);
        }
        if x_target_bounds.0 <= x_position && x_position <= x_target_bounds.1 {
            return starting_x_velocity;
        } else {
            starting_x_velocity += 1;
        }
    }
}

fn find_max_y_velocity(y_target_bounds: (i64, i64)) -> i64 {
    let mut starting_y_velocity = 1;
    let mut max_y_starting_velocity = 0;
    for _ in 0..1000 {
        let mut y_position = 0;
        let mut current_y_velocity = starting_y_velocity;
        for _ in 0..1000 {
            let (Position(_, new_y_position), Velocity(_, new_y_velocity)) =
                tick(Position(0, y_position), Velocity(0, current_y_velocity));
            y_position = new_y_position;
            current_y_velocity = new_y_velocity;
            if y_position >= y_target_bounds.0 && y_position <= y_target_bounds.1 {
                max_y_starting_velocity = cmp::max(starting_y_velocity, max_y_starting_velocity);
                break;
            }
        }
        starting_y_velocity += 1;
    }
    max_y_starting_velocity
}

pub fn day_17_problem_1() -> io::Result<i64> {
    let x_target_bounds = (209, 238);
    let y_target_bounds = (-86, -59);

    let min_x_velocity = find_min_x_velocity(x_target_bounds);
    let max_y_velocity = find_max_y_velocity(y_target_bounds);

    let mut position = Position(0, 0);
    let mut velocity = Velocity(min_x_velocity, max_y_velocity);
    let mut max_pos = 0;
    while velocity.1 != 0 {
        let (next_pos, next_velocity) = tick(position, velocity);
        max_pos = cmp::max(max_pos, next_pos.1);
        position = next_pos;
        velocity = next_velocity;
    }
    Ok(max_pos)
}
