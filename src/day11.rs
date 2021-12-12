use itertools::{iproduct, Itertools};
use pathfinding::prelude::{bfs_reach, Matrix};

fn apply_neighbours(m: &mut Matrix<u8>) -> u64 {
    for k in iproduct!(0..m.rows, 0..m.columns) {
        m[&k] += 1;
        if m[&k] == 10 {
            bfs_reach(k, |n| {
                m.neighbours(n, true)
                    .filter(|k| {
                        m[k] += 1;
                        m[k] == 10
                    })
                    .collect_vec()
            })
            .count(); // consume somehow
        }
    }
    m.iter_mut()
        .map(|a| {
            if *a >= 10 {
                *a = 0;
                1
            } else {
                0
            }
        })
        .sum()
}

fn count_flashes_for_100_steps(input: &str) -> u64 {
    let mut m: Matrix<u8> = input.lines().map(|c| c.bytes().map(|x| x - b'0')).collect();
    (0..100).map(|_| apply_neighbours(&mut m)).sum::<u64>()
}

fn detect_first_flash_of_all_cylce(input: &str) -> u64 {
    let mut m: Matrix<u8> = input.lines().map(|c| c.bytes().map(|x| x - b'0')).collect();
    (1..).find(|_| apply_neighbours(&mut m) == 100).unwrap() as u64
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1() {
        let buf = std::fs::read_to_string("puzzleData/day11.txt").unwrap();
        assert_eq!(1721, count_flashes_for_100_steps(&buf));
    }
    #[test]
    fn part1_test() {
        assert_eq!(1656, count_flashes_for_100_steps(TESTDATA));
    }

    #[test]
    fn part2() {
        let buf = std::fs::read_to_string("puzzleData/day11.txt").unwrap();
        assert_eq!(298, detect_first_flash_of_all_cylce(&buf));
    }

    #[test]
    fn part2_test() {
        assert_eq!(195, detect_first_flash_of_all_cylce(TESTDATA));
    }
    const TESTDATA: &'static str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
}
