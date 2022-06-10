use pathfinding::directed::dijkstra::dijkstra;

const NEIGHBOURS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn main() {
    let map: Vec<Vec<u32>> = include_str!("./input.txt")
        .lines()
        .map(|line| line.chars().map(|d| d.to_digit(10).unwrap()).collect())
        .collect();

    let (_, distance) = dijkstra(
        &(0, 0),
        |(x, y)| {
            NEIGHBOURS
                .iter()
                .filter_map(|(dx, dy)| {
                    map.get((*y + *dy) as usize)
                        .and_then(|row| row.get((*x + *dx) as usize))
                        .and_then(|weight| Some(((*x + *dx, *y + *dy), *weight as usize)))
                })
                .collect::<Vec<_>>()
        },
        |point| *point == (99, 99),
    )
    .unwrap();

    println!("Answear: {:?}", distance);
}
