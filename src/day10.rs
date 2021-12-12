use std::io::Read;

use itertools::Itertools;
use simple_lines::ReadExt;

fn parse_line(input: &str) -> Result<(), Error> {
    let mut buf = vec![];
    for char in input.chars() {
        match char {
            '<' => buf.push('>'),
            '(' => buf.push(')'),
            '[' => buf.push(']'),
            '{' => buf.push('}'),
            i @ ('>' | ')' | '}' | ']') => match buf.pop() {
                Some(x) => {
                    if x != i {
                        return Err(Error::CorruptLine(i));
                    }
                }
                None => return Err(Error::InvalidChar(i)),
            },
            a => return Err(Error::InvalidChar(a)),
        }
    }
    if !buf.is_empty() {
        return Err(Error::Incomplete(buf));
    }

    Ok(())
}

fn get_error_score_corrupt(r: impl Read) -> i32 {
    r.lines_rc()
        .filter_map(|line| {
            if let Err(Error::CorruptLine(i)) = parse_line(&line.unwrap()) {
                Some(match i {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    _ => 25137,
                })
            } else {
                None
            }
        })
        .sum::<i32>()
}

fn get_error_score_incomplete(r: impl Read) -> u64 {
    let mut sorted = r
        .lines_rc()
        .filter_map(|line| {
            if let Err(Error::Incomplete(i)) = parse_line(&line.unwrap()) {
                Some(
                    i.iter()
                        .rev()
                        .map(|x| match x {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            _ => 4,
                        })
                        .fold(0, |acc, n| acc * 5 + n),
                )
            } else {
                None
            }
        })
        .collect_vec();
    sorted.sort_unstable();

    sorted[sorted.len() / 2]
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid point format")]
    InvalidChar(char),

    #[error("Line is corrupt")]
    CorruptLine(char),

    #[error("Remainings")]
    Incomplete(Vec<char>),
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1() {
        let result = get_error_score_corrupt(std::fs::File::open("puzzleData/day10.txt").unwrap());
        assert_eq!(370407, result);
    }

    #[test]
    fn part1_test() {
        let result = get_error_score_corrupt(std::io::Cursor::new(TEST_INPUT));
        assert_eq!(26397, result);
    }

    #[test]
    fn part2() {
        let result =
            get_error_score_incomplete(std::fs::File::open("puzzleData/day10.txt").unwrap());
        assert_eq!(3249889609, result);
    }

    #[test]
    fn part2_test() {
        let result = get_error_score_incomplete(std::io::Cursor::new(TEST_INPUT));
        assert_eq!(288957, result);
    }

    const TEST_INPUT: &'static str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
}
