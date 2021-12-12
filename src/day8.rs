use std::{collections::HashMap, io::Read};

use itertools::process_results;
use simple_lines::ReadExt;

fn count_unique(r: impl Read) -> Result<u32, Box<dyn std::error::Error>> {
    Ok(process_results(r.lines_rc(), |iter| {
        iter.map(|line| {
            line.split(" | ")
                .nth(1)
                .expect("Invalid input contains no | ")
                .split(' ')
                .filter(|c| matches!(c.len(), 2 | 3 | 4 | 7))
                .count() as u32
        })
        .sum::<u32>()
    })?)
}

const TILES: [u8; 10] = [
    Tile::Top as u8 //0
        | Tile::TopLeft as u8
        | Tile::TopRight as u8
        | Tile::BottomLeft as u8
        | Tile::BottomRight as u8
        | Tile::Bottom as u8,
    Tile::TopRight as u8 | Tile::BottomRight as u8, //1
    Tile::Top as u8 //2
        | Tile::TopRight as u8
        | Tile::Center as u8
        | Tile::BottomLeft as u8
        | Tile::Bottom as u8,
    Tile::Top as u8 // 3
        | Tile::TopRight as u8
        | Tile::Center as u8
        | Tile::BottomRight as u8
        | Tile::Bottom as u8,
    Tile::TopLeft as u8 | Tile::TopRight as u8 | Tile::Center as u8 | Tile::BottomRight as u8, // 4
    Tile::Top as u8 // 5
        | Tile::TopLeft as u8
        | Tile::Center as u8
        | Tile::BottomRight as u8
        | Tile::Bottom as u8,
    Tile::Top as u8 // 6
        | Tile::TopLeft as u8
        | Tile::Center as u8
        | Tile::BottomRight as u8
        | Tile::BottomLeft as u8
        | Tile::Bottom as u8,
    Tile::Top as u8 | Tile::TopRight as u8 | Tile::BottomRight as u8, // 7
    Tile::Top as u8 // 8
        | Tile::TopLeft as u8
        | Tile::TopRight as u8
        | Tile::Center as u8
        | Tile::BottomLeft as u8
        | Tile::BottomRight as u8
        | Tile::Bottom as u8,
    Tile::Top as u8 // 9
        | Tile::TopLeft as u8
        | Tile::TopRight as u8
        | Tile::Center as u8
        | Tile::BottomRight as u8
        | Tile::Bottom as u8,
];

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum Tile {
    Top = 1,
    TopLeft = 2,
    TopRight = 4,
    Center = 8,
    BottomLeft = 16,
    BottomRight = 32,
    Bottom = 64,
}

fn number_from_tiles(input: u8) -> Option<u8> {
    TILES.iter().position(|x| *x == input).map(|x| x as u8)
}

fn solve(permutations: &str, input: &str) -> u64 {
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut four = "";
    let mut one = "";
    let mut seven = "";
    let mut eight = "";
    for group in permutations.split(' ') {
        for char in group.chars() {
            *map.entry(char).or_default() += 1;
        }
        match group.len() {
            2 => one = group,
            3 => seven = group,
            4 => four = group,
            7 => eight = group,
            _ => {}
        }
    }

    let mut solutions = map
        .iter()
        .filter_map(|(char, count)| match count {
            4 => Some((*char, Tile::BottomLeft)),
            6 => Some((*char, Tile::TopLeft)),
            9 => Some((*char, Tile::BottomRight)),
            _ => None,
        })
        .collect::<HashMap<_, _>>();

    let top_right = one.chars().find(|c| !solutions.contains_key(c)).unwrap();
    solutions.insert(top_right, Tile::TopRight);
    let center_char = four.chars().find(|c| !solutions.contains_key(c)).unwrap();
    solutions.insert(center_char, Tile::Center);
    let top_char = seven.chars().find(|c| !solutions.contains_key(c)).unwrap();
    solutions.insert(top_char, Tile::Top);
    let bottom_char = eight.chars().find(|c| !solutions.contains_key(c)).unwrap();
    solutions.insert(bottom_char, Tile::Bottom);

    input
        .split(' ')
        .map(|digit| {
            number_from_tiles(
                digit
                    .chars()
                    .fold(0, |acc, c| acc | *solutions.get(&c).unwrap() as u8),
            )
            .unwrap() as u64
        })
        .fold(0, |acc, n| acc * 10 + n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            504,
            count_unique(std::fs::File::open("puzzleData/day8.txt").unwrap()).unwrap()
        )
    }
    #[test]
    fn part1_test() {
        assert_eq!(
            26,
            count_unique(std::fs::File::open("puzzleData/day8_test.txt").unwrap()).unwrap()
        )
    }

    #[test]
    fn part2() {
        assert_eq!(
            1073431,
            std::fs::File::open("puzzleData/day8.txt")
                .unwrap()
                .lines_rc()
                .map(|line| {
                    let line = line.unwrap();
                    let mut parts = line.split(" | ");
                    solve(parts.next().unwrap(), parts.next().unwrap())
                })
                .sum::<u64>()
        )
    }

    #[test]
    fn test_single_line() {
        let line = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab";
        let input = "cdfeb fcadb cdfeb cdbaf";
        assert_eq!(5353, solve(line, input));
    }

    #[test]
    fn number_from_tiles_match() {
        assert_eq!(
            Some(0),
            number_from_tiles(
                Tile::Top as u8
                    | Tile::TopLeft as u8
                    | Tile::TopRight as u8
                    | Tile::BottomLeft as u8
                    | Tile::BottomRight as u8
                    | Tile::Bottom as u8
            )
        );

        assert_eq!(
            Some(1),
            number_from_tiles(Tile::TopRight as u8 | Tile::BottomRight as u8)
        );
        assert_eq!(
            Some(2),
            number_from_tiles(
                Tile::Top as u8
                    | Tile::TopRight as u8
                    | Tile::Center as u8
                    | Tile::BottomLeft as u8
                    | Tile::Bottom as u8
            )
        );
        assert_eq!(
            Some(3),
            number_from_tiles(
                Tile::Top as u8
                    | Tile::TopRight as u8
                    | Tile::Center as u8
                    | Tile::BottomRight as u8
                    | Tile::Bottom as u8
            )
        );

        assert_eq!(
            Some(4),
            number_from_tiles(
                Tile::TopRight as u8
                    | Tile::TopLeft as u8
                    | Tile::Center as u8
                    | Tile::BottomRight as u8
            )
        );
        assert_eq!(
            Some(5),
            number_from_tiles(
                Tile::Top as u8
                    | Tile::TopLeft as u8
                    | Tile::Center as u8
                    | Tile::BottomRight as u8
                    | Tile::Bottom as u8
            )
        );
        assert_eq!(
            Some(6),
            number_from_tiles(
                Tile::Top as u8
                    | Tile::TopLeft as u8
                    | Tile::Center as u8
                    | Tile::BottomRight as u8
                    | Tile::BottomLeft as u8
                    | Tile::Bottom as u8
            )
        );

        assert_eq!(
            Some(7),
            number_from_tiles(Tile::Top as u8 | Tile::TopRight as u8 | Tile::BottomRight as u8)
        );

        assert_eq!(
            Some(8),
            number_from_tiles(
                Tile::Top as u8
                    | Tile::TopLeft as u8
                    | Tile::TopRight as u8
                    | Tile::Center as u8
                    | Tile::BottomRight as u8
                    | Tile::BottomLeft as u8
                    | Tile::Bottom as u8
            )
        );

        assert_eq!(
            Some(9),
            number_from_tiles(
                Tile::Top as u8
                    | Tile::TopLeft as u8
                    | Tile::TopRight as u8
                    | Tile::Center as u8
                    | Tile::BottomRight as u8
                    | Tile::Bottom as u8
            )
        );
    }
}
