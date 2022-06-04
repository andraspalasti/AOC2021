const STEPS: usize = 100;

const NEIGHBOURS: [(isize, isize); 8] = [
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
    (1, 0),
    (0, 1),
    (0, -1),
    (-1, 0),
];

fn flash(map: &mut Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    map[y][x] = 0;
    NEIGHBOURS
        .iter()
        .map(|(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
        .fold(1, |acc, (x, y)| {
            match map.get_mut(y).and_then(|row| row.get_mut(x)) {
                Some(value) if 0 < *value => {
                    *value += 1;
                    acc + (9 < *value).then(|| flash(map, x, y)).unwrap_or(0)
                }
                _ => acc,
            }
        })
}

pub fn main() {
    let mut map: Vec<Vec<u32>> = include_str!("./input.txt")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut flashes = 0;

    for _ in 0..STEPS {
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                map[y][x] += 1;
            }
        }

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if 9 < map[y][x] {
                    flashes += flash(&mut map, x, y);
                }
            }
        }
    }

    println!("Answear: {}", flashes);
}
