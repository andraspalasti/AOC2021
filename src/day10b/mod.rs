const OPENING: &str = "([{<";
const CLOSING: &str = ")]}>";
const POINTS: [usize; 4] = [1, 2, 3, 4];

pub fn main() {
    let mut scores: Vec<usize> = include_str!("./input.txt")
        .lines()
        .filter_map(|line| {
            let mut stack: Vec<char> = Vec::new();

            for c in line.chars() {
                if OPENING.contains(c) {
                    stack.push(c);
                } else if CLOSING.contains(c) {
                    let latest = stack.pop().unwrap();
                    if !is_pair(latest, c) {
                        // Corrupted line
                        return None;
                    }
                }
            }

            Some(stack.iter().rev().fold(0, |score, c| score * 5 + point(*c)))
        })
        .collect();

    scores.sort();
    println!("Answear: {}", scores[scores.len() / 2]);
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
