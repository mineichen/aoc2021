use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

use crate::utils::AocError;
use itertools::{process_results, Itertools};
use simple_lines::ReadExt;

fn parse(r: impl Read) -> Result<Game, Box<dyn std::error::Error>> {
    Ok(process_results(r.lines_rc(), |mut lines| {
        let numbers: Result<Vec<i32>, Box<dyn std::error::Error>> = lines
            .next()
            .ok_or(AocError::EmptyInput)?
            .split(',')
            .map(|p| Ok(p.parse()?))
            .collect();
        let numbers = numbers?;

        let boards: Result<Vec<_>, _> = lines
            .skip(1)
            .batching(|it| {
                let numbers: Result<Vec<i32>, Box<dyn std::error::Error>> = it
                    .take_while(|line| line.trim().len() > 0)
                    .flat_map(|line| {
                        line.split(' ')
                            .filter_map(|maybe_no| {
                                (maybe_no.len() != 0).then(|| Ok(maybe_no.parse::<i32>()?))
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect();
                match numbers {
                    Ok(numbers) => (numbers.len() > 0).then(|| Ok(Board { numbers })),
                    Err(e) => Some(Err(e)),
                }
            })
            .collect();

        Ok((numbers, boards?).into()) as Result<_, Box<dyn std::error::Error>>
    })??)
}

struct Board {
    numbers: Vec<i32>,
}

struct Game {
    boards: Vec<HashSet<i32>>,
    numbers: Vec<i32>,
    i_to_occupancies: HashMap<i32, Vec<(usize, u8, u8)>>,
    occupancies: Vec<u8>,
}

#[derive(Debug)]
enum Error {
    AllNumbersAreUsed,
}

impl Game {
    fn play_all(mut self) -> Result<(i32, Vec<i32>), Error> {
        let mut extracted_numbers = vec![];
        std::mem::swap(&mut self.numbers, &mut extracted_numbers);
        for number in extracted_numbers.into_iter() {
            if let Some((_, result)) = self.play(number).into_iter().next() {
                return Ok(result);
            }
        }

        Err(Error::AllNumbersAreUsed)
    }

    fn find_worst_board(mut self) -> Result<(i32, Vec<i32>), Error> {
        let mut extracted_numbers = vec![];
        std::mem::swap(&mut self.numbers, &mut extracted_numbers);
        let mut remaining_boards = self.boards.len();
        let mut finish_states = vec![false; self.boards.len()];
        for number in extracted_numbers.into_iter() {
            for (pos, result) in self.play(number) {
                if finish_states[pos] == false {
                    remaining_boards -= 1;
                    finish_states[pos] = true;
                    if remaining_boards == 0 {
                        return Ok(result);
                    }
                }
            }
        }

        Err(Error::AllNumbersAreUsed)
    }

    fn play(&mut self, number: i32) -> Vec<(usize, (i32, Vec<i32>))> {
        let a = Vec::new();
        let mut number_items = self.i_to_occupancies.get(&number);
        let items = number_items.get_or_insert(&a);

        let mut result = vec![];
        for r in items.iter().filter_map(|(board, col, row)| {
            let board_ref = &mut self.boards[*board];
            board_ref.remove(&number);
            let col_occupancies = &mut self.occupancies[*board * 10 + *col as usize];
            *col_occupancies += 1;
            let col_occupancies = *col_occupancies;
            let row_occupancies = &mut self.occupancies[*board * 10 + 5 + *row as usize];
            *row_occupancies += 1;
            if col_occupancies == 5 || *row_occupancies == 5 {
                Some((*board, (number, board_ref.iter().copied().collect())))
            } else {
                None
            }
        }) {
            result.push(r);
        }

        result
    }
}

impl From<(Vec<i32>, Vec<Board>)> for Game {
    fn from((numbers, boards): (Vec<i32>, Vec<Board>)) -> Self {
        let occupancies = vec![0; boards.len() * 10];
        let mut i_to_occupancies = HashMap::<i32, Vec<(usize, u8, u8)>, _>::new();
        for (i, board) in boards.iter().enumerate() {
            for (field_pos, field) in board.numbers.iter().enumerate() {
                let entry = i_to_occupancies.entry(*field).or_default();
                let col = field_pos as u8 % 5;
                let row = field_pos as u8 / 5;
                entry.push((i, col, row));
            }
        }
        Self {
            numbers,
            boards: boards
                .into_iter()
                .map(|i| i.numbers.into_iter().collect())
                .collect(),
            occupancies,
            i_to_occupancies,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    #[test]
    fn part1_test() {
        let puzzle = super::parse(Cursor::new(TEST_INPUT)).unwrap();
        let (number, remainings) = puzzle.play_all().unwrap();
        let sum = remainings.into_iter().sum();
        assert_eq!(24, number);
        assert_eq!(188, sum);
        assert_eq!(4512, number * sum)
    }

    #[test]
    fn part1() {
        let puzzle = super::parse(std::fs::File::open("puzzleData/day4.txt").unwrap()).unwrap();
        let (number, remainings) = puzzle.play_all().unwrap();
        let sum: i32 = remainings.into_iter().sum();
        assert_eq!(60368, number * sum)
    }

    #[test]
    fn part2_test() {
        let puzzle = super::parse(Cursor::new(TEST_INPUT)).unwrap();
        let (number, remainings) = puzzle.find_worst_board().unwrap();
        let sum = remainings.into_iter().sum();
        assert_eq!(13, number);
        assert_eq!(148, sum);
    }

    #[test]
    fn part2() {
        let puzzle = super::parse(std::fs::File::open("puzzleData/day4.txt").unwrap()).unwrap();
        let (number, remainings) = puzzle.find_worst_board().unwrap();
        let sum: i32 = remainings.into_iter().sum();
        assert_eq!(17435, number * sum)
    }

    const TEST_INPUT: &'static str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
}
