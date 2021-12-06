mod day_1_problem_1;
mod day_1_problem_2;
mod day_2_problem_1;
mod day_2_problem_2;
mod day_3_problem_1;
mod day_3_problem_2;
mod day_4_problem_1;
mod day_4_problem_2;
fn main() {
    match day_1_problem_1::day_1_problem_1() {
        Ok(result) => {
            println!("Day 1 problem 1: {:?}", result);
        }
        Err(err) => {
            println!("There was an error! {:?}", err);
        }
    }
    match day_1_problem_2::day_1_problem_2() {
        Ok(result) => {
            println!("Day 1 problem 2: {:?}", result);
        }
        Err(err) => {
            println!("There was an error! {:?}", err);
        }
    }
    match day_2_problem_1::day_2_problem_1() {
        Ok(result) => {
            println!("Day 2 problem 1: {:?}", result);
        }
        Err(err) => {
            println!("There was an error! {:?}", err);
        }
    }
    match day_2_problem_2::day_2_problem_2() {
        Ok(result) => {
            println!("Day 2 problem 2: {:?}", result);
        }
        Err(err) => {
            println!("There was an error! {:?}", err);
        }
    }
    match day_3_problem_1::day_3_problem_1() {
        Ok(result) => {
            println!("Day 3 problem 1: {:?}", result);
        }
        Err(err) => {
            println!("There was an error! {:?}", err);
        }
    }
    match day_3_problem_2::day_3_problem_2() {
        Ok(result) => {
            println!("Day 3 problem 2: {:?}", result);
        }
        Err(err) => {
            println!("There was an error! {:?}", err);
        }
    }
    match day_4_problem_1::day_4_problem_1() {
        Ok(result) => {
            println!("Day 4 problem 1: {:?}", result);
        }
        Err(err) => {
            println!("There was an error! {:?}", err);
        }
    }
    match day_4_problem_2::day_4_problem_2() {
        Ok(result) => {
            println!("Day 4 problem 2: {:?}", result);
        }
        Err(err) => {
            println!("There was an error! {:?}", err);
        }
    }



}
