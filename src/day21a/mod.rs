pub fn main() {
    let (mut pos, mut score): ([_; 2], _) = (
        include_str!("./input.txt")
            .lines()
            .map(|line| line.split_once(": ").unwrap().1.parse::<u16>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
        [0; 2],
    );

    let mut dice = (1..=100).cycle();
    let mut roll = || dice.next().unwrap();

    for turn in 0.. {
        for p in 0..2 {
            pos[p] = 1 + ((pos[p] + roll() + roll() + roll() - 1) % 10);
            score[p] += pos[p];

            if 1000 <= score[p] {
                let loser_score = score[(p + 1) % 2] as usize;
                let rolls = (turn * 2 + p + 1) * 3;
                println!("Answear: {}", loser_score * rolls);
                return;
            }
        }
    }
}
