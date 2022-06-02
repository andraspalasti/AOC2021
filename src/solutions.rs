use std::collections::HashMap;

use crate::{
    day10a, day10b, day1a, day1b, day2a, day2b, day3a, day3b, day4a, day4b, day5a, day5b, day6a,
    day6b, day9a, day9b,
};

pub fn solutions() -> HashMap<&'static str, fn()> {
    HashMap::from([
        ("day1a", day1a::main as fn()),
        ("day1b", day1b::main as fn()),
        ("day2a", day2a::main as fn()),
        ("day2b", day2b::main as fn()),
        ("day3a", day3a::main as fn()),
        ("day3b", day3b::main as fn()),
        ("day4a", day4a::main as fn()),
        ("day4b", day4b::main as fn()),
        ("day5a", day5a::main as fn()),
        ("day5b", day5b::main as fn()),
        ("day6a", day6a::main as fn()),
        ("day6b", day6b::main as fn()),
        ("day9a", day9a::main as fn()),
        ("day9b", day9b::main as fn()),
        ("day10a", day10a::main as fn()),
        ("day10b", day10b::main as fn()),
    ])
}
