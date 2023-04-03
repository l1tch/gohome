use std::env;

mod go_home;
pub use go_home::*;

fn numbers_from_arg() -> Vec<usize> {
    let mut nums: Vec<usize> = Vec::new();

    for argument in env::args() {
        let argument: usize = match argument.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        nums.push(argument);
    }

    return nums;
}

fn main() {
    let o_arr = go_home(numbers_from_arg());

    println!("{:?}", o_arr);
}
