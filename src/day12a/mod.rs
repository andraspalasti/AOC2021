use std::collections::HashMap;

fn walk<'a>(graph: &HashMap<&'a str, Vec<&'a str>>, path: &mut Vec<&'a str>) -> usize {
    if let Some(cur) = path.last().map(|s| *s) {
        if cur == "end" {
            return 1;
        }

        graph[cur].iter().fold(0, |acc, neighbour| {
            if is_small(*neighbour) && path.contains(neighbour) {
                acc
            } else {
                path.push(*neighbour);
                let paths = walk(graph, path);
                path.pop();
                acc + paths
            }
        })
    } else {
        0
    }
}

pub fn main() {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in include_str!("./input.txt").lines() {
        let (node_a, node_b) = line.split_once('-').unwrap();

        // add the nodes and connect them
        graph.entry(node_a).or_default().push(node_b);
        graph.entry(node_b).or_default().push(node_a);
    }

    let paths = walk(&graph, &mut vec!["start"]);

    println!("Answear: {}", paths);
}

fn is_small(cave: &str) -> bool {
    for c in cave.chars() {
        if c.is_uppercase() {
            return false;
        }
    }
    true
}
