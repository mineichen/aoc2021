use itertools::{process_results, Itertools};
use simple_lines::ReadExt;
use std::fs::File;

pub fn count_increments(input: impl IntoIterator<Item = usize>) -> usize {
    input
        .into_iter()
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

pub fn maybe_panic_count_increments_in_file(path: &str) -> usize {
    let file = std::fs::File::open(path).expect("File always exists");
    let numbers = file.lines_rc().map(|line| {
        line.expect("no invalid string")
            .parse()
            .expect("line is a number")
    });
    count_increments(numbers)
}

// No collect ignore errors
pub fn sloppy_count_increments_in_file(path: &str) -> usize {
    if let Ok(file) = std::fs::File::open(path) {
        let numbers = file
            .lines_rc()
            //.filter_map(Result::ok)
            .filter_map(|e| e.ok())
            .filter_map(|line| line.parse().ok());
        count_increments(numbers)
    } else {
        0
    }
}

pub fn count_increments_in_file_collect(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let numbers = file
        .lines_rc()
        .map(|line| Ok(line?.parse()?))
        .collect::<Result<Vec<usize>, Box<dyn std::error::Error>>>()?;
    Ok(count_increments(numbers))
}

pub fn count_increments_in_file(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let numbers = file.lines_rc().map(|line| Ok(line?.parse()?));
    process_results(numbers, |iter| count_increments(iter))
}

pub fn count_window_increments(input: impl IntoIterator<Item = usize>) -> usize {
    let sums = input.into_iter().tuple_windows().map(|(a, b, c)| a + b + c);
    count_increments(sums)
}

pub fn count_window_increments_in_file(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let numbers = file.lines_rc().map(|line| Ok(line?.parse()?));
    process_results(numbers, |iter| count_window_increments(iter))
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: [usize; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    #[test]
    fn input_part1() {
        assert_eq!(
            1688,
            super::count_increments_in_file("puzzleData/day1.txt").unwrap()
        );
    }
    #[test]
    fn test_input_part1() {
        assert_eq!(7, super::count_increments(TEST_INPUT));
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(5, super::count_window_increments(TEST_INPUT))
    }

    #[test]
    fn input_part2() {
        assert_eq!(
            1728,
            super::count_window_increments_in_file("puzzleData/day1.txt").unwrap()
        )
    }
}
