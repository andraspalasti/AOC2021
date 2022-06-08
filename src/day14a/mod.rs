use std::collections::HashMap;

const STEPS: usize = 10;

pub fn main() {
    let mut lines = include_str!("./input.txt").lines();

    let mut template: Vec<char> = lines.next().unwrap().chars().collect();

    let rules: HashMap<(char, char), char> = HashMap::from_iter(lines.skip(1).map(|line| {
        let (l, r) = line.split_once(" -> ").unwrap();
        (
            (l.as_bytes()[0] as char, l.as_bytes()[1] as char),
            r.as_bytes()[0] as char,
        )
    }));

    let mut counts: HashMap<char, usize> = HashMap::new();

    for el in &template {
        let count = counts.entry(*el).or_insert(0);
        *count += 1;
    }

    for _ in 0..STEPS {
        let mut tmp = vec![*template.first().unwrap()];
        for i in 1..template.len() {
            if let Some(new_el) = rules.get(&(template[i - 1], template[i])) {
                let count = counts.entry(*new_el).or_insert(0);
                *count += 1;
                tmp.push(*new_el);
            }

            tmp.push(template[i]);
        }

        template = tmp;
    }

    println!(
        "Answear: {}",
        counts.values().max().unwrap() - counts.values().min().unwrap()
    );
}
