type Velocity = (i32, i32);

fn hit(mut v: Velocity, ((min_x, max_x), (min_y, max_y)): ((i32, i32), (i32, i32))) -> bool {
    let mut pos = (0, 0);
    loop {
        if min_x <= pos.0 && pos.0 <= max_x && min_y <= pos.1 && pos.1 <= max_y {
            return true;
        }

        if max_x < pos.0 {
            return false;
        }

        if v.0 == 0 && (pos.0 < min_x || max_x < pos.0) {
            return false;
        }

        if v.1 < 0 && pos.1 < min_y {
            return false;
        }

        pos.0 += v.0;
        pos.1 += v.1;

        if 0 < v.0 {
            v.0 -= 1;
        }
        v.1 -= 1;
    }
}

pub fn main() {
    let target: Vec<(i32, i32)> = include_str!("./input.txt")
        .trim()
        .trim_start_matches("target area: x=")
        .splitn(2, ", y=")
        .map(|part| part.split_once("..").unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();
    let target = (target[0], target[1]);

    let mut count = 0;
    for vx in 1..=target.0 .1 {
        for vy in -1000..=1000 {
            if hit((vx, vy), target) {
                count += 1;
            }
        }
    }

    println!("Answear: {}", count);
}
