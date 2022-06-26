#[derive(Clone, Copy)]
enum Pixel {
    Light,
    Dark,
}

impl From<char> for Pixel {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Light,
            '.' => Self::Dark,
            _ => unreachable!(),
        }
    }
}

struct Picture {
    pixels: Vec<Vec<Pixel>>,
    rest: Pixel,
}

impl Picture {
    fn enhance(&self, algorithm: &[Pixel]) -> Self {
        let mut new = Picture {
            pixels: vec![vec![Pixel::Dark; self.width() + 2]; self.height() + 2],
            rest: match self.rest {
                Pixel::Light => algorithm[511],
                Pixel::Dark => algorithm[0],
            },
        };

        for y in 0..new.pixels.len() {
            for x in 0..new.pixels[y].len() {
                new.pixels[y][x] = algorithm[self.block((x as i32 - 1, y as i32 - 1))];
            }
        }

        new
    }

    fn block(&self, center: (i32, i32)) -> usize {
        let (min_x, max_x) = (center.0 - 1, center.0 + 1);
        let (min_y, max_y) = (center.1 - 1, center.1 + 1);

        let mut sum = 0;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                sum = sum << 1;
                match self
                    .pixels
                    .get(y as usize)
                    .map(|row| row.get(x as usize).unwrap_or(&self.rest))
                    .unwrap_or(&self.rest)
                {
                    &Pixel::Light => sum += 1,
                    &Pixel::Dark => sum += 0,
                }
            }
        }
        sum
    }

    fn height(&self) -> usize {
        self.pixels.len()
    }

    fn width(&self) -> usize {
        if 0 < self.pixels.len() {
            self.pixels[0].len()
        } else {
            0
        }
    }
}

pub fn main() {
    let (algorithm, mut picture): (Vec<Pixel>, Picture) = include_str!("./input.txt")
        .split_once("\n\n")
        .map(|(algorithm, image)| {
            (
                algorithm.chars().map(|c| Pixel::from(c)).collect(),
                Picture {
                    pixels: image
                        .lines()
                        .map(|line| line.chars().map(|c| Pixel::from(c)).collect())
                        .collect(),
                    rest: Pixel::Dark,
                },
            )
        })
        .unwrap();

    picture = picture.enhance(&algorithm);
    picture = picture.enhance(&algorithm);

    // the count of the light pixels
    let count = picture
        .pixels
        .iter()
        .flatten()
        .fold(0, |acc, pixel| match pixel {
            &Pixel::Light => acc + 1,
            &Pixel::Dark => acc,
        });

    println!("Answear: {}", count);
}
