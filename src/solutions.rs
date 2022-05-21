use std::collections::HashMap;

use crate::{day1a, day1b, day2a, day2b, day3a, day3b};

pub fn solutions() -> HashMap<&'static str, fn()> {
    HashMap::from([
        ("day1a", day1a::main as fn()),
        ("day1b", day1b::main as fn()),
        ("day2a", day2a::main as fn()),
        ("day2b", day2b::main as fn()),
        ("day3a", day3a::main as fn()),
        ("day3b", day3b::main as fn()),
    ])
}
