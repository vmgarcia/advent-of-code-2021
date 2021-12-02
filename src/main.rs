mod problem_1;
mod problem_2;
mod problem_3;
mod problem_4;

fn main() {
    match problem_1::problem_1() {
        Ok(result) => {
            println!("Problem 1: {:?}", result);
        }
        Err(err) => {
            println!("There was an error! {:?}", err);
        }
    }
    match problem_2::problem_2() {
        Ok(result) => {
            println!("Problem 2: {:?}", result);
        }
        Err(err) => {
            println!("There was an error! {:?}", err);
        }
    }
    match problem_3::problem_3() {
        Ok(result) => {
            println!("Problem 3: {:?}", result);
        }
        Err(err) => {
            println!("There was an error! {:?}", err);
        }
    }
    match problem_4::problem_4() {
        Ok(result) => {
            println!("Problem 4: {:?}", result);
        }
        Err(err) => {
            println!("There was an error! {:?}", err);
        }
    }
}