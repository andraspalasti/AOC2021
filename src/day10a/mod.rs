const OPENING: &str = "([{<";
const CLOSING: &str = ")]}>";
const POINTS: [usize; 4] = [3, 57, 1197, 25137];

pub fn main() {
    let score: usize = include_str!("./input.txt")
        .lines()
        .map(|line| {
            let mut stack: Vec<char> = vec![];
            for c in line.chars() {
                if OPENING.contains(c) {
                    stack.push(c);
                } else if CLOSING.contains(c) {
                    let latest = stack.pop().unwrap();

                    // Corrupted line
                    if !is_pair(latest, c) {
                        return point(c);
                    }
                }
            }
            0
        })
        .sum();

    println!("Answear: {}", score);
}

fn is_pair(a: char, b: char) -> bool {
    a == pair(b)
}

fn pair(c: char) -> char {
    if let Some(i) = OPENING.find(c) {
        return CLOSING.chars().nth(i).unwrap();
    }
    if let Some(i) = CLOSING.find(c) {
        return OPENING.chars().nth(i).unwrap();
    }
    unreachable!("Unsupported char '{}'", c)
}

fn point(c: char) -> usize {
    let idx = OPENING.find(c).unwrap_or_else(|| CLOSING.find(c).unwrap());
    POINTS[idx]
}
