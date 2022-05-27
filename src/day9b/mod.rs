use std::{collections::HashMap, str::FromStr, string::ParseError};

struct Map {
    heights: HashMap<(i32, i32), u32>,
}

impl Map {
    fn basin_size(&mut self, point: &(i32, i32)) -> usize {
        match self.heights.get_mut(point) {
            Some(height) if *height < 9 => {
                *height = 9;
                [(0, 1), (1, 0), (0, -1), (-1, 0)]
                    .iter()
                    .map(|(dx, dy)| self.basin_size(&(point.0 + dx, point.1 + dy)))
                    .sum::<usize>()
                    + 1
            }
            _ => 0,
        }
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
    let mut map = Map::from_str(include_str!("./input.txt")).unwrap();

    // Get maximum of coordinates
    let (max_x, max_y) = map.heights.keys().fold((0, 0), |max, cur| {
        (std::cmp::max(max.0, cur.0), std::cmp::max(max.1, cur.1))
    });

    let mut sizes = vec![];
    for x in 0..=max_x {
        for y in 0..=max_y {
            let point = (x, y);
            if map.heights.get(&point).unwrap() == &9 {
                continue;
            }

            sizes.push(map.basin_size(&point));
        }
    }
    sizes.sort();

    println!(
        "Answear: {:?}",
        sizes.iter().rev().take(3).fold(1, |acc, size| acc * size)
    );
}
