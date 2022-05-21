const WIDTH: u32 = 12;

fn reduce(mut nums: Vec<u32>, pred: fn(i32, i32) -> bool) -> u32 {
    for col in 0..WIDTH {
        if nums.len() < 1 {
            unreachable!("The reduction of a vec should result in exactly one value")
        }

        if nums.len() == 1 {
            return nums[0];
        }

        let (mut ones, mut zeros) = (0, 0);
        let idx = 1 << (WIDTH - col - 1);
        for n in &nums {
            if n & idx == 0 {
                zeros += 1;
            } else {
                ones += 1;
            }
        }

        // determines whether to keep the zeros or the ones
        let keep_zeros = pred(zeros, ones);
        nums.retain(|n| match keep_zeros {
            true => n & idx == 0,
            false => n & idx != 0,
        });
    }
    nums[0]
}

pub fn main() {
    let nums: Vec<u32> = include_str!("input.txt")
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();

    // oxygen generator rating
    let ogr = reduce(
        nums.clone(),
        |zeros, ones| ones < zeros,
    );

    // CO2 scrubber rating
    let csr = reduce(
        nums.clone(),
        |zeros, ones| zeros <= ones,
    );

    println!("Answear: {}", ogr * csr);
}
