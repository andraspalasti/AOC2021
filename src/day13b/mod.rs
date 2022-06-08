use std::{collections::HashSet, str::FromStr, string::ParseError};

#[derive(Copy, Clone)]
enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

impl FromStr for Fold {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.trim_start_matches("fold along ").split_once('=').unwrap();

        match left {
            "y" => Ok(Self::Horizontal(right.parse().unwrap())),
            "x" => Ok(Self::Vertical(right.parse().unwrap())),
            _ => unreachable!("Unhandled axis '{}'", left),
        }
    }
}

fn fold(points: &mut Vec<(usize, usize)>, fold: Fold) {
    match fold {
        Fold::Horizontal(y) => {
            for point in points {
                if y < point.1 {
                    point.1 = y - (point.1 - y);
                }
            }
        }
        Fold::Vertical(x) => {
            for point in points {
                if x < point.0 {
                    point.0 = x - (point.0 - x);
                }
            }
        }
    }
}

pub fn main() {
    let (points, folds) = include_str!("./input.txt").split_once("\n\n").unwrap();

    let mut points: Vec<(usize, usize)> = points
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect();

    let folds: Vec<Fold> = folds.lines().map(|line| line.parse().unwrap()).collect();

    for f in folds {
        fold(&mut points, f);
    }

    let points: HashSet<(usize, usize)> = HashSet::from_iter(points);
    let (max_x, max_y) = points.iter().fold((0, 0), |acc, point| {
        (
            if acc.0 < point.0 { point.0 } else { acc.0 },
            if acc.1 < point.1 { point.1 } else { acc.1 },
        )
    });

    println!("Answear: ");
    for y in 0..=max_y {
        for x in 0..=max_x {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    // println!("Answear: {}", points.len());
}
