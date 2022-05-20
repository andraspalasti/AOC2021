use std::{str::FromStr, string::ParseError};

enum Dir {
    Forward,
    Up,
    Down,
}

struct Command {
    dir: Dir,
    amount: usize,
}

impl FromStr for Command {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amount) = s.split_once(' ').unwrap();
        Ok(Command {
            dir: match dir {
                "forward" => Dir::Forward,
                "up" => Dir::Up,
                "down" => Dir::Down,
                _ => unreachable!(),
            },
            amount: amount.parse().unwrap(),
        })
    }
}

pub fn main() {
    let commands: Vec<Command> = include_str!("./input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let (mut depth, mut horizontal, mut aim) = (0, 0, 0);
    for cmd in commands {
        match cmd.dir {
            Dir::Forward => {
                horizontal += cmd.amount;
                depth += cmd.amount * aim;
            }
            Dir::Up => aim -= cmd.amount,
            Dir::Down => aim += cmd.amount,
        }
    }

    println!("Answear: {}", depth * horizontal);
}
