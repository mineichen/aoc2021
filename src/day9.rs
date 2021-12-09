use std::{collections::HashMap, io::Read};

use itertools::{process_results, Itertools};
use simple_lines::ReadExt;

fn parse(r: impl Read) -> Result<Puzzle, Box<dyn std::error::Error>> {
    let mut data = Vec::new();
    let mut iter = r.lines_rc();
    data.extend(
        iter.next()
            .ok_or(crate::utils::AocError::EmptyInput)??
            .chars()
            .map(|c| (c as u8) - '0' as u8),
    );
    let mut width = data.len();
    for line in iter {
        data.extend(line?.chars().map(|c| (c as u8) - '0' as u8));
    }
    let height = data.len() / width;
    Ok(Puzzle {
        width,
        data,
        height,
    })
}

impl Puzzle {
    fn find_low_points(&self) -> Vec<usize> {
        let Puzzle {
            data,
            width,
            height,
        } = self;

        let mut min_indices = Vec::new();
        let mut xy = 0;
        for y in 0..*height {
            for x in 0..*width {
                let cur = data[xy];
                let top_min = if y == 0 {
                    false
                } else {
                    data[xy - width] <= cur
                };
                let left_min = if x == 0 { false } else { data[xy - 1] <= cur };
                let bottom_min = if y == height - 1 {
                    false
                } else {
                    data[xy + width] <= cur
                };
                let right_min = if x == width - 1 {
                    false
                } else {
                    data[xy + 1] <= cur
                };

                if !(top_min || left_min || bottom_min || right_min) {
                    min_indices.push(xy);
                }
                xy += 1;
            }
        }

        min_indices
    }

    fn low_point_risk_level(&self) -> i32 {
        self.find_low_points()
            .into_iter()
            .map(|idx| self.data[idx] as i32 + 1)
            .sum::<i32>()
    }

    fn flood_fill(&mut self, i: usize) -> i32 {
        if self.data[i] == 9 {
            return 0;
        }
        self.data[i] = 9;
        let x = i % self.width;
        let y = i / self.width;
        let mut count = 1;
        if x > 0 {
            count += self.flood_fill(i - 1)
        }
        if y > 0 {
            count += self.flood_fill(i - self.width);
        }
        if x < self.width - 1 {
            count += self.flood_fill(i + 1)
        }
        if y < self.height - 1 {
            count += self.flood_fill(i + self.width);
        }
        count
    }
    fn top_lake_sizes(mut self, n: usize) -> i32 {
        self.find_low_points()
            .iter()
            .map(|i| self.flood_fill(*i))
            .sorted_by(|a, b| b.cmp(&a))
            .take(n)
            .product::<i32>()
    }
}

struct Puzzle {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let puzzle = parse(std::fs::File::open("puzzleData/day9.txt").unwrap()).unwrap();
        assert_eq!(15, puzzle.low_point_risk_level())
    }
    #[test]
    fn part1_test() {
        let puzzle = parse(std::io::Cursor::new(TESTDATA)).unwrap();
        assert_eq!(15, puzzle.low_point_risk_level())
    }

    #[test]
    fn part1_lowpoints() {
        let puzzle = parse(std::io::Cursor::new(TESTDATA)).unwrap();
        assert_eq!(vec!(1, 9, 22, 46), puzzle.find_low_points())
    }

    #[test]
    fn part2_test() {
        let puzzle = parse(std::io::Cursor::new(TESTDATA)).unwrap();
        assert_eq!(1134, puzzle.top_lake_sizes(3));
    }

    #[test]
    fn part2() {
        let puzzle = parse(std::fs::File::open("puzzleData/day9.txt").unwrap()).unwrap();
        assert_eq!(1134, puzzle.top_lake_sizes(3));
    }

    const TESTDATA: &'static str = "2199943210
3987894921
9856789892
8767896789
9899965678";
}
