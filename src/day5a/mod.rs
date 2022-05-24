use std::{collections::HashMap, str::FromStr, string::ParseError};

pub type Point = (i32, i32);

pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> Point {
        self.start
    }

    pub fn end(&self) -> Point {
        self.end
    }
}

impl FromStr for Line {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").unwrap();
        let start = start.split_once(',').unwrap();
        let end = end.split_once(',').unwrap();
        Ok(Self::new(
            (start.0.parse().unwrap(), start.1.parse().unwrap()),
            (end.0.parse().unwrap(), end.1.parse().unwrap()),
        ))
    }
}

pub struct LinePoints {
    line: Line,
    d: i32,
}

impl Iterator for LinePoints {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        let step_x = match self.line.start.0.cmp(&self.line.end.0) {
            std::cmp::Ordering::Greater => -1,
            std::cmp::Ordering::Less => 1,
            _ => 0,
        };

        let step_y = match self.line.start.1.cmp(&self.line.end.1) {
            std::cmp::Ordering::Greater => -1,
            std::cmp::Ordering::Less => 1,
            _ => 0,
        };

        let next = (
            self.line.start.0 + self.d * step_x,
            self.line.start.1 + self.d * step_y,
        );
        self.d += 1;

        if next != (self.line.end.0 + step_x, self.line.end.1 + step_y) {
            Some(next)
        } else {
            None
        }
    }
}

impl IntoIterator for Line {
    type Item = Point;
    type IntoIter = LinePoints;
    fn into_iter(self) -> Self::IntoIter {
        LinePoints { line: self, d: 0 }
    }
}

pub fn main() {
    let lines: Vec<Line> = include_str!("./input.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .filter(|line: &Line| line.start().0 == line.end().0 || line.start().1 == line.end().1)
        .collect();

    let mut map: HashMap<Point, usize> = HashMap::new();

    for line in lines {
        for point in line {
            let count = map.entry(point).or_insert(0);
            *count += 1;
        }
    }

    let overlaps = map.iter().filter(|(_, count)| 1 < **count).count();

    println!("Answear: {}", overlaps);
}

#[cfg(test)]
mod tests {
    use super::{Line, Point};

    #[test]
    fn line_points() {
        assert_eq!(
            Line::new((0, 0), (0, 2))
                .into_iter()
                .collect::<Vec<Point>>(),
            vec![(0, 0), (0, 1), (0, 2)]
        );

        assert_eq!(
            Line::new((0, 0), (2, 2))
                .into_iter()
                .collect::<Vec<Point>>(),
            vec![(0, 0), (1, 1), (2, 2)]
        );

        assert_eq!(
            Line::new((0, 0), (2, 0))
                .into_iter()
                .collect::<Vec<Point>>(),
            vec![(0, 0), (1, 0), (2, 0)]
        );
    }
}
