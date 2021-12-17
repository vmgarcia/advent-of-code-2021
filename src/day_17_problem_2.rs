use itertools::Itertools;
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

fn find_all_velocities(x_bounds: (i64, i64), y_bounds: (i64, i64)) -> HashSet<(i64, i64)> {
    let mut velocities = HashSet::new();
    for x in 0..400 {
        for y in -400..400 {
            let mut position = Position(0, 0);
            let mut velocity = Velocity(x, y);
            for _ in 0..500 {
                if y_bounds.0 <= position.1
                    && position.1 <= y_bounds.1
                    && x_bounds.0 <= position.0
                    && position.0 <= x_bounds.1
                {
                    velocities.insert((x, y));
                    break;
                }

                let (
                    Position(new_x_position, new_y_position),
                    Velocity(new_x_velocity, new_y_velocity),
                ) = tick(position, velocity);

                position = Position(new_x_position, new_y_position);
                velocity = Velocity(new_x_velocity, new_y_velocity);
            }
        }
    }
    velocities
}

pub fn day_17_problem_2() -> io::Result<usize> {
    let x_bounds = (209, 238);
    let y_bounds = (-86, -59);

    let all_velocities = find_all_velocities(x_bounds, y_bounds);
    Ok(all_velocities.len())
}
