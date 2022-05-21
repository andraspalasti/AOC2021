/// Statistics
#[derive(Debug)]
struct Stat {
    ones: u32,
    zeros: u32,
}

impl Stat {
    /// Most common value, returns a one or a zero
    fn most(self: &Self) -> u32 {
        if self.zeros < self.ones {
            1
        } else {
            0
        }
    }

    /// Least common value, returns a one or a zero
    fn least(self: &Self) -> u32 {
        if self.zeros < self.ones {
            0
        } else {
            1
        }
    }
}

pub fn main() {
    let lines: Vec<&str> = include_str!("./input.txt").lines().collect();

    let mut stats: Vec<Stat> = vec![];

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            let stat = match stats.get_mut(i) {
                Some(stat) => stat,
                None => {
                    stats.push(Stat { ones: 0, zeros: 0 });
                    stats.get_mut(i).unwrap()
                }
            };

            match c {
                '1' => stat.ones += 1,
                '0' => stat.zeros += 1,
                _ => unreachable!("There should only be 1s and 0s in a line"),
            }
        }
    }

    // gamma rate
    let gr = stats.iter().fold(0, |v, stat| (v << 1) + stat.most());

    // epsilon rate
    let er = stats.iter().fold(0, |v, stat| (v << 1) + stat.least());

    println!("Answear: {}", gr * er);
}
