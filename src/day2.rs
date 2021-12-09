use std::{io::Read, str::FromStr};

use itertools::process_results;
use simple_lines::ReadExt;

enum Command {
    Up(i32),
    Down(i32),
    Forward(i32)
}


#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Invalid Command: {0}")]
    ParseCommand(String),
    #[error("Not enough arguments")]
    InvalidNumberOfArguments(&'static str),
    #[error("Not enough arguments")]
    ValueIsNotANumber()
}


impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let command = parts.next().ok_or(Error::InvalidNumberOfArguments("command"))?;
        let number = parts.next().ok_or(Error::InvalidNumberOfArguments("number"))?.parse().map_err(|_| Error::ValueIsNotANumber())?;
        match command {
            "forward" => Ok(Command::Forward(number)),
            "up" => Ok(Command::Up(number)),
            "down" => Ok(Command::Down(number)),
            _ => Err(Error::ParseCommand(command.to_owned()).into())
        }
    }
}


#[derive(Debug, Default)]
struct Position {
    x: i32,
    y: i32
}

fn navigate(input: impl Iterator<Item=Command>) -> Position {
    let mut position = Position::default();

    for command in input {
        match command {
            Command::Up(y) => position.y -= y,
            Command::Down(y) => position.y += y,
            Command::Forward(x) => position.x += x,
        }
    }

    position
}

fn navigate_reader(input: impl Read) -> Result<Position, Box<dyn std::error::Error>> {
    let data = input.lines_rc().map(|x| Ok(x?.parse()?));
    process_results(data, |iter| navigate(iter))
}

#[derive(Debug, Default)]
struct PositionV2 {
    x: i32,
    y: i32,
    aim: i32
}

fn navigate_v2(input: impl Iterator<Item=Command>) -> PositionV2 {
    let mut position = PositionV2::default();
    for command in input {
        match command {
            Command::Up(y) => position.aim -= y,
            Command::Down(y) => position.aim += y,
            Command::Forward(x) => {
                position.x += x;
                position.y += position.aim * x;
            },
        }
    }
    position
}

fn navigate_reader_v2(input: impl Read) -> Result<PositionV2, Box<dyn std::error::Error>> {
    let data = input.lines_rc().map(|x| Ok(x?.parse()?));
    process_results(data, |iter| navigate_v2(iter))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::Cursor, fs::File};

    #[test]
    fn test_part1() {
        let test_input = Cursor::new("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");
        let pos = navigate_reader(test_input).unwrap();
        assert_eq!(150, pos.x * pos.y);
    }

    #[test]
    fn part1() {
        let test_input = File::open("puzzleData\\day2.txt").unwrap();
        let pos = navigate_reader(test_input).unwrap();
        assert_eq!(1694130, pos.x * pos.y);
    }

    #[test]
    fn test_part2() {
        let test_input = Cursor::new("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");
        let pos = navigate_reader_v2(test_input).unwrap();
        assert_eq!(900, pos.x * pos.y);
    }

    #[test]
    fn part2() {
        let test_input = File::open("puzzleData\\day2.txt").unwrap();
        let pos = navigate_reader_v2(test_input).unwrap();
        assert_eq!(1698850445, pos.x * pos.y);
    }
}
