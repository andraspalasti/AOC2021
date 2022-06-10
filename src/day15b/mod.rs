use pathfinding::directed::dijkstra::dijkstra;

const NEIGHBOURS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn main() {
    let map: Vec<Vec<u32>> = include_str!("./input.txt")
        .lines()
        .map(|line| line.chars().map(|d| d.to_digit(10).unwrap()).collect())
        .collect();

    let size = map.len();
    let goal = (size as i32 * 5 - 1, size as i32 * 5 - 1);

    let (_, distance) = dijkstra(
        &(0, 0),
        |&(x, y)| {
            NEIGHBOURS
                .iter()
                .map(|&(dx, dy)| ((x + dx) as usize, (y + dy) as usize))
                .filter(|&(x, y)| x / 5 < size && y / 5 < size)
                .filter_map(|(x, y)| {
                    let risk = map.get(y % size)?.get(x % size)?;
                    Some((
                        (x as i32, y as i32),
                        (((x / size) + (y / size) + *risk as usize - 1) % 9) + 1,
                    ))
                })
                .collect::<Vec<_>>()
        },
        |&point| point == goal,
    )
    .unwrap();

    println!("Answear: {:?}", distance);
}
