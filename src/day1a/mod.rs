pub fn main() {
    let numbers: Vec<i32> = include_str!("./input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut increases = 0;

    for i in 1..numbers.len() {
        if numbers[i - 1] < numbers[i] {
            increases += 1;
        }
    }

    println!("Answear: {}", increases);
}
