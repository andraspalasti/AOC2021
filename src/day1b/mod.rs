pub fn main() {
    let numbers: Vec<i32> = include_str!("./input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut increases = 0;

    for i in 3..numbers.len() {
        let left: i32 = numbers[(i - 3)..=(i - 1)].iter().sum();
        let right: i32 = numbers[(i - 2)..=i].iter().sum();

        if left < right {
            increases += 1;
        }
    }

    println!("Answear: {}", increases);
}
