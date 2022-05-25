// the number of possible states that a fish can be in
const STATES: usize = 9;

// the number of days that we are simulating
const DAYS: usize = 256;

pub fn main() {
    let fishes: Vec<usize> = include_str!("./input.txt")
        .trim()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let mut states = vec![0 as usize; STATES];
    for fish in fishes {
        states[fish] += 1;
    }

    for _ in 0..DAYS {
        states.rotate_left(1);
        states[6] += states[8];
    }

    println!("Answear: {}", states.iter().sum::<usize>());
}
