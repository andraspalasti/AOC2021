mod solutions;

mod day10a;
mod day10b;
mod day11a;
mod day11b;
mod day12a;
mod day13a;
mod day13b;
mod day14a;
mod day15a;
mod day15b;
mod day17a;
mod day17b;
mod day18a;
mod day18b;
mod day19a;
mod day19b;
mod day1a;
mod day1b;
mod day20a;
mod day20b;
mod day21a;
mod day21b;
mod day22a;
mod day22b;
mod day2a;
mod day2b;
mod day3a;
mod day3b;
mod day4a;
mod day4b;
mod day5a;
mod day5b;
mod day6a;
mod day6b;
mod day9a;
mod day9b;

use std::env;

use solutions::solutions;

fn main() -> Result<(), String> {
    let mut args: Vec<String> = env::args().collect();

    // shift vec to the left
    args.rotate_left(1);
    args.pop();

    if args.len() < 1 {
        return Err(
            "You have to provide a command line argument to select which solution to run"
                .to_string(),
        );
    }

    let solutions = solutions();

    for arg in args {
        if let Some(solution) = solutions.get(arg.as_str()) {
            solution();
        } else {
            return Err(format!("Could not find solution for `{}`", arg));
        }
    }

    Ok(())
}
