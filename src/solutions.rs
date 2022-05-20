use std::collections::HashMap;

use crate::{day1a, day1b};

pub fn solutions() -> HashMap<&'static str, fn()> {
    HashMap::from([
        ("day1a", day1a::main as fn()),
        ("day1b", day1b::main as fn()),
    ])
}
