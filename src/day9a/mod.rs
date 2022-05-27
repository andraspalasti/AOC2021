use std::{collections::HashMap, str::FromStr, string::ParseError};

struct Map {
    heights: HashMap<(i32, i32), u32>,
}

impl Map {
    fn is_low(&self, point: &(i32, i32)) -> bool {
        let moves = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let height = self.heights.get(&point).unwrap();

        for (x, y) in moves {
            match self.heights.get(&(x + point.0, y + point.1)) {
                Some(h) if h <= height => return false,
                _ => continue,
            }
        }
        true
    }

    fn risk(&self, point: &(i32, i32)) -> u32 {
        self.heights.get(&point).unwrap() + 1
    }
}

impl FromStr for Map {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut heights = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                heights.insert((x as i32, y as i32), c.to_digit(10).unwrap());
            }
        }
        Ok(Self { heights })
    }
}

pub fn main() {
    let map = Map::from_str(include_str!("./input.txt")).unwrap();

    let mut risks = 0;
    for point in map.heights.keys() {
        if map.is_low(point) {
            risks += map.risk(point);
        }
    }

    println!("Answear: {}", risks)
}
