use itertools::{process_results, FoldWhile, Itertools};
use simple_lines::ReadExt;
use std::{io::Read, rc::Rc};

struct FuelStats {
    gamma: u32,
    epsilon: u32,
}

struct LifeStats {
    co2: u32,
    oxygen: u32,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("empty input")]
    EmptyInput,
    #[error("unknown char")]
    UnknownChar(char),
    #[error("No Rating")]
    CalculateRatingFailed,
}

fn build_accumulator(
    input: impl Iterator<Item = Rc<String>>,
) -> Result<(u32, Vec<u32>), Box<dyn std::error::Error>> {
    let mut input = input.peekable();
    let first = input.peek().ok_or(Error::EmptyInput)?;
    let mut accumulator = vec![0; first.len()];
    let mut count = 0;

    for line in input {
        count += 1;
        for (char, acc) in line.chars().zip(accumulator.iter_mut()) {
            match char {
                '0' => continue,
                '1' => *acc += 1,
                _ => Err(Error::UnknownChar(char))?,
            }
        }
    }
    Ok((count, accumulator))
}

fn parse(r: impl Read) -> Result<FuelStats, Box<dyn std::error::Error>> {
    let lines = r.lines_rc();
    let (count, accumulator) = process_results(lines, |nrs| build_accumulator(nrs))??;

    let half_count = count / 2;
    let acc_len = accumulator.len();
    let epsilon = accumulator
        .into_iter()
        .fold(0, |acc, next| (acc << 1) + (next > half_count) as u32);

    Ok(FuelStats {
        epsilon,
        gamma: epsilon ^ !(u32::max_value() << acc_len),
    })
}

fn parse_oxygen_and_co2(r: impl Read) -> Result<LifeStats, Box<dyn std::error::Error>> {
    let mut peekable = r.lines_rc().peekable();
    let line_len = peekable
        .peek()
        .ok_or(Error::EmptyInput)?
        .as_ref()
        .map_err(|_| Error::EmptyInput)?
        .len();
    let all: Result<Vec<_>, Box<dyn std::error::Error>> =
        peekable.map(|l| Ok(u32::from_str_radix(&l?, 2)?)).collect();
    let data = &mut all?[..];

    let oxygen = generate_rating(line_len, data, |a, b| a > b)?;
    let co2 = generate_rating(line_len, data, |a, b| a <= b)?;

    Ok(LifeStats { oxygen, co2 })
}

/// Determiner: Fn(count_zeros, count_ones) -> take_zeros?
fn generate_rating(
    line_len: usize,
    data: &mut [u32],
    determiner: impl Fn(usize, usize) -> bool,
) -> Result<u32, Box<dyn std::error::Error>> {
    let result = (0..line_len).rev().fold_while(data, |acc, shifts| {
        // dbg!("{:?}", acc.iter().map(|f| format!("{:b}", f)).collect::<Vec<_>>());
        let next = partition_by_digit(acc, shifts, &determiner);
        if next.len() == 1 {
            FoldWhile::Done(next)
        } else {
            FoldWhile::Continue(next)
        }
    });
    match result {
        FoldWhile::Continue(_) => Err(Error::CalculateRatingFailed)?,
        FoldWhile::Done(x) => Ok(*x
            .first()
            .expect("FoldWhile::Done only returns Done if slice.len() == 1")),
    }
}

fn partition_by_digit<'a>(
    data: &'a mut [u32],
    shifts: usize,
    determiner: &impl Fn(usize, usize) -> bool,
) -> &'a mut [u32] {
    let pos = itertools::partition(data.iter_mut(), |x| ((*x >> shifts) & 1) == 0);
    let (a, b) = data.split_at_mut(pos);
    if (determiner)(a.len(), b.len()) {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    const TEST_DATA: &'static str =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
    #[test]
    fn test_data() {
        let input = std::io::Cursor::new(TEST_DATA);
        let result = super::parse(input).unwrap();
        assert_eq!(198, result.epsilon * result.gamma);
    }

    #[test]
    fn part1() {
        let input = std::fs::File::open("puzzleData/day3.txt").unwrap();
        let result = super::parse(input).unwrap();
        assert_eq!(4138664, result.epsilon * result.gamma);
    }

    #[test]
    fn test_part2() {
        let input = std::io::Cursor::new(TEST_DATA);
        let result = super::parse_oxygen_and_co2(input).unwrap();
        assert_eq!(23, result.oxygen);
        assert_eq!(10, result.co2);
    }

    #[test]
    fn part2() {
        let input = std::fs::File::open("puzzleData/day3.txt").unwrap();
        let result = super::parse_oxygen_and_co2(input).unwrap();
        assert_eq!(4273224, result.oxygen * result.co2);
    }

    #[test]
    fn partition_test() {
        let mut raw = [0b1100, 0b0101, 0b1000, 0b0000];
        let partition = super::partition_by_digit(&mut raw[..], 3, &|a, b| a >= b);
        assert_eq!(2, partition.len());
    }
}
