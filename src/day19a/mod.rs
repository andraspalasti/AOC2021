use std::collections::{HashMap, HashSet};

use nalgebra::{Matrix3, Vector3};

const OVERLAPS: usize = 12;

const BASES: [Matrix3<i32>; 24] = [
    Matrix3::new(1, 0, 0, 0, 1, 0, 0, 0, 1),
    Matrix3::new(1, 0, 0, 0, 0, 1, 0, -1, 0),
    Matrix3::new(1, 0, 0, 0, -1, 0, 0, 0, -1),
    Matrix3::new(1, 0, 0, 0, 0, -1, 0, 1, 0),
    Matrix3::new(0, 1, 0, 0, 0, 1, 1, 0, 0),
    Matrix3::new(0, 1, 0, 1, 0, 0, 0, 0, -1),
    Matrix3::new(0, 1, 0, 0, 0, -1, -1, 0, 0),
    Matrix3::new(0, 1, 0, -1, 0, 0, 0, 0, 1),
    Matrix3::new(0, 0, 1, 1, 0, 0, 0, 1, 0),
    Matrix3::new(0, 0, 1, 0, 1, 0, -1, 0, 0),
    Matrix3::new(0, 0, 1, -1, 0, 0, 0, -1, 0),
    Matrix3::new(0, 0, 1, 0, -1, 0, 1, 0, 0),
    Matrix3::new(-1, 0, 0, 0, -1, 0, 0, 0, 1),
    Matrix3::new(-1, 0, 0, 0, 0, 1, 0, 1, 0),
    Matrix3::new(-1, 0, 0, 0, 1, 0, 0, 0, -1),
    Matrix3::new(-1, 0, 0, 0, 0, -1, 0, -1, 0),
    Matrix3::new(0, -1, 0, 0, 0, -1, 1, 0, 0),
    Matrix3::new(0, -1, 0, 1, 0, 0, 0, 0, 1),
    Matrix3::new(0, -1, 0, 0, 0, 1, -1, 0, 0),
    Matrix3::new(0, -1, 0, -1, 0, 0, 0, 0, -1),
    Matrix3::new(0, 0, -1, -1, 0, 0, 0, 1, 0),
    Matrix3::new(0, 0, -1, 0, 1, 0, 1, 0, 0),
    Matrix3::new(0, 0, -1, 1, 0, 0, 0, -1, 0),
    Matrix3::new(0, 0, -1, 0, -1, 0, -1, 0, 0),
];

type Beacon = Vector3<i32>;
type Scanner = Vec<Beacon>;

pub fn main() {
    // Unfinished scanners that are not yet discovered
    let mut unfinished: Vec<_> = include_str!("./input.txt")
        .lines()
        .filter(|&line| line != "")
        .fold(Vec::new() as Vec<Scanner>, |mut scanners, line| {
            if line.starts_with("--- scanner") {
                scanners.push(Vec::new());
                scanners
            } else {
                scanners.last_mut().unwrap().push(Vector3::from_iterator(
                    line.split(",").filter_map(|s| s.parse().ok()),
                ));
                scanners
            }
        });

    // The discovered and correctly rotated scanners
    let mut finished = vec![unfinished.pop().unwrap()];
    let mut latest_discovered = 0;

    while latest_discovered < finished.len() {
        unfinished.retain(|scanner| {
            for basis in BASES {
                let mut transformed: Vec<Beacon> =
                    scanner.iter().map(|beacon| basis * beacon).collect();

                if let Some(scanner_pos) = overlaps(&finished[latest_discovered], &transformed) {
                    transformed
                        .iter_mut()
                        .for_each(|beacon| *beacon += scanner_pos);
                    finished.push(transformed);
                    return false;
                }
            }
            true
        });

        latest_discovered += 1;
    }

    let beacons: HashSet<Beacon> = HashSet::from_iter(finished.into_iter().flatten());
    println!("Answear: {}", beacons.len());
}

/// Checks the overlaps relative to scanner `a` and if there is enough than gives back the position
/// of scanner `b` relative to `a`
fn overlaps(a: &Scanner, b: &Scanner) -> Option<Vector3<i32>> {
    // the suspected positions of scanner b relative to scanner a
    let mut positions: HashMap<Beacon, usize> = HashMap::new();

    for beacon_a in a {
        for beacon_b in b {
            let count = positions.entry(beacon_a - beacon_b).or_insert(0);
            *count += 1;
        }
    }

    positions
        .iter()
        .find(|&(_, &count)| OVERLAPS <= count)
        .map(|record| *record.0)
}
