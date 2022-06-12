/// Snailfish sequence: [(val, depth)]
type SFNumber = Vec<(u32, u32)>;

pub fn main() {
    let numbers: Vec<SFNumber> = include_str!("./input.txt")
        .trim()
        .lines()
        .map(|line| {
            let mut depth = 0;
            line.chars().fold(vec![], |mut number, c| {
                match c {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    c if c.is_digit(10) => number.push((c.to_digit(10).unwrap(), depth)),
                    _ => (),
                };
                number
            })
        })
        .collect();

    let mut sum = numbers
        .into_iter()
        .reduce(|sum, number| {
            let mut sum = add(&sum, &number);
            reduce(&mut sum);
            sum
        })
        .unwrap();

    println!("Answear: {}", magnitude(&mut sum));
}

fn add(a: &SFNumber, b: &SFNumber) -> SFNumber {
    a.iter()
        .chain(b.iter())
        .map(|&(val, depth)| (val, depth + 1))
        .collect()
}

fn magnitude(num: &mut SFNumber) -> u32 {
    while 2 < num.len() {
        for i in 1..num.len() {
            let (left, right) = (num[i - 1], num[i]);
            if left.1 == right.1 {
                num[i - 1] = (3 * left.0 + 2 * right.0, left.1 - 1);
                num.remove(i);
                break;
            }
        }
    }
    3 * num[0].0 + 2 * num[1].0
}

fn reduce(num: &mut SFNumber) {
    // check for explodes
    for i in 1..num.len() {
        let (left, right) = (num[i - 1], num[i]);
        if left.1 == right.1 && 4 < left.1 {
            num.get_mut(i.wrapping_sub(2)).map(|val| val.0 += left.0);
            num.get_mut(i + 1).map(|val| val.0 += right.0);
            num[i - 1] = (0, left.1 - 1);
            num.remove(i);
            return reduce(num);
        }
    }

    // check for splits
    for i in 0..num.len() {
        let (val, depth) = num[i];
        if 9 < val {
            num.insert(i + 1, ((val + 1) / 2, depth + 1));
            num[i] = (val / 2, depth + 1);
            return reduce(num);
        }
    }
}
