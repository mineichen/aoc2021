use std::{io::Read, num::ParseIntError, str::FromStr};

use itertools::{process_results};
use simple_lines::ReadExt;

fn parse(r: impl Read) -> impl Iterator<Item = Result<Line, Box<dyn std::error::Error>>> {
    r.lines_rc().map(|l| Ok(l?.parse()?))
}

fn count_horizontal_and_vertical_intersections(
    r: impl Read,
) -> Result<u32, Box<dyn std::error::Error>> {
    process_results(parse(r), |lines| {
        count_intersections(lines.filter(|l| l.is_horizontal_or_vertical()).collect())
    })
}

fn count_all_intersections(r: impl Read) -> Result<u32, Box<dyn std::error::Error>> {
    process_results(parse(r), |lines| count_intersections(lines.collect()))
}

fn count_intersections(lines: Vec<Line>) -> u32 {
    let max = get_max(lines.iter().copied());
    let x_bound = max.x + 1;
    let mut field = vec![0; (x_bound * (max.y + 1)) as usize];
    for line in lines.into_iter() {
        let x_incr = match line.0.x.cmp(&line.1.x) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
        let y_incr = match line.0.y.cmp(&line.1.y) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };

        let (mut x, mut y) = (line.0.x, line.0.y);
        loop {
            field[(x + y * x_bound) as usize] += 1;
            x += x_incr;
            y += y_incr;
            if x == line.1.x && y == line.1.y {
                field[(x + y * x_bound) as usize] += 1;
                break;
            }
        }
    }
    /*
    for line in field .chunks(10) {
        for no in line {
            print!("{}", no);
        }
        print!("\n");

    } */

    field.into_iter().filter(|x| *x >= 2).count() as u32
}

fn get_max(lines: impl IntoIterator<Item = Line>) -> Point {
    lines.into_iter().fold(
        Point {
            x: i32::min_value(),
            y: i32::min_value(),
        },
        |acc, next| Point {
            x: acc.x.max(next.0.x).max(next.1.x),
            y: acc.y.max(next.0.y).max(next.1.y),
        },
    )
}

#[derive(Debug, thiserror::Error)]
enum ParseError {
    #[error("invalid point format")]
    InvalidPoint,
    #[error("invalid line format")]
    InvalidLine,
    #[error("parseInt Error")]
    ParseIntError(#[from] ParseIntError),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Line(Point, Point);

impl Line {
    fn is_horizontal_or_vertical(&self) -> bool {
        self.0.x == self.1.x || self.0.y == self.1.y
    }
}

impl FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");
        Ok(Line(
            parts.next().ok_or(ParseError::InvalidLine)?.parse()?,
            parts.next().ok_or(ParseError::InvalidLine)?.parse()?,
        ))
    }
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        Ok(Point {
            x: parts.next().ok_or(ParseError::InvalidPoint)?.parse()?,
            y: parts.next().ok_or(ParseError::InvalidPoint)?.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(
            5,
            count_horizontal_and_vertical_intersections(std::io::Cursor::new(TEST_INPUT)).unwrap()
        )
    }

    #[test]
    fn part1() {
        assert_eq!(
            6189,
            count_horizontal_and_vertical_intersections(
                std::fs::File::open("puzzleData/day5.txt").unwrap()
            )
            .unwrap()
        )
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            12,
            count_all_intersections(std::io::Cursor::new(TEST_INPUT)).unwrap()
        )
    }

    #[test]
    fn part2() {
        assert_eq!(
            19164,
            count_all_intersections(std::fs::File::open("puzzleData/day5.txt").unwrap()).unwrap()
        )
    }

    const TEST_INPUT: &'static str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
}
