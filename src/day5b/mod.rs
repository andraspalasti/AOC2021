use std::collections::HashMap;

use super::day5a::{Line, Point};

pub fn main() {
    let lines: Vec<Line> = include_str!("./input.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut map: HashMap<Point, usize> = HashMap::new();

    for line in lines {
        for point in line {
            let count = map.entry(point).or_insert(0);
            *count += 1;
        }
    }

    let overlaps = map.iter().filter(|(_, count)| 1 < **count).count();

    println!("Answear: {}", overlaps);
}
