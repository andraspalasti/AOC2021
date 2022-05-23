use std::{num::ParseIntError, str::FromStr};

// Width and height of bingo board
const WIDTH: usize = 5;
const HEIGHT: usize = 5;

#[derive(Debug)]
struct Board {
    numbers: [[(u32, bool); WIDTH]; HEIGHT],
    last_marked: u32,
}

impl Board {
    fn new() -> Self {
        Self {
            numbers: [[(0, false); WIDTH]; HEIGHT],
            last_marked: 0,
        }
    }

    fn mark(&mut self, n: u32) {
        for row in &mut self.numbers {
            for (num, marked) in row {
                if *num == n {
                    *marked = true;
                    self.last_marked = n;
                }
            }
        }
    }

    fn bingo(&self) -> bool {
        for i in 0..HEIGHT {
            // vertical and horizontal marks
            let (mut v_marks, mut h_marks) = (0, 0);
            for j in 0..WIDTH {
                if self.numbers[i][j].1 {
                    h_marks += 1;
                }
                if self.numbers[j][i].1 {
                    v_marks += 1;
                }
            }

            if h_marks == HEIGHT || v_marks == WIDTH {
                return true;
            }
        }
        false
    }

    fn score(&self) -> u32 {
        self.numbers
            .iter()
            .flatten()
            .filter_map(|(num, marked)| match marked {
                true => None,
                false => Some(num),
            })
            .sum::<u32>()
            * self.last_marked
    }
}

impl FromStr for Board {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Self::new();

        let (mut x, mut y) = (0, 0);
        for num in s.split_whitespace() {
            board.numbers[y][x].0 = num.parse()?;

            x += 1;
            if WIDTH <= x {
                x = 0;
                y += 1;
            }
        }
        Ok(board)
    }
}

pub fn main() {
    let mut lines = include_str!("./input.txt").split("\n\n");
    let nums: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = lines.map(|board| board.parse().unwrap()).collect();

    for n in nums {
        boards.retain(|board| !board.bingo());

        if boards.len() == 0 {
            break;
        }

        for board in &mut boards {
            board.mark(n);
        }

        if boards.len() == 1 && boards.first().unwrap().bingo() {
            break;
        }

    }

    match boards.first() {
        Some(board) => println!("Answear: {}", board.score()),
        None => println!("Could not find the board that wins last"),
    }
}
