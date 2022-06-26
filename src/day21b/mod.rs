/// Player type: (position, score)
type Player = (u16, u16);

pub fn main() {
    let players: [Player; 2] = include_str!("./input.txt")
        .lines()
        .map(|line| (line.split_once(": ").unwrap().1.parse().unwrap(), 0))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let wins = universes(players, 0);
    println!("Answear: {}", wins.0.max(wins.1));
}

fn universes(players: [Player; 2], turn: usize) -> (usize, usize) {
    if 21 <= players[0].1 {
        return (1, 0);
    }

    if 21 <= players[1].1 {
        return (0, 1);
    }

    let mut wins = (0, 0);
    for (amount, weight) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let mut new = players.clone();
        new[turn].0 = 1 + ((new[turn].0 + amount - 1) % 10);
        new[turn].1 += new[turn].0;

        let result = universes(new, (turn + 1) % 2);

        wins.0 += result.0 * weight;
        wins.1 += result.1 * weight;
    }
    wins
}
