use itertools::{iproduct, Itertools};
use pathfinding::prelude::{bfs_reach, Matrix};

fn lows(m: &Matrix<u8>) -> impl Iterator<Item = (usize, usize)> + '_ {
    iproduct!(0..m.rows, 0..m.columns).filter(|&k| m.neighbours(&k, false).all(|n| m[&n] > m[&k]))
}

fn calculate_risk_level(input: &str) -> u32 {    
    let m = input.lines().map(|c| c.bytes()).collect();        
    lows(&m).map(|k| (m[&k] - b'0') as u32 + 1).sum::<u32>()
}

fn count_top_lake_tiles(input: &str, n: usize) -> usize {
    let m = input.lines().map(|c| c.bytes()).collect();
    lows(&m)
        .map(|n| {
            bfs_reach(n, |&n| {
                m.neighbours(&n, false)
                    .filter(|&k| m[&k] != b'9' && m[&k] > m[&n])
                    .collect_vec()
            })
            .count()
        })
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(n)
        .product()
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::Read;

    #[test]
    fn part1() {
        let mut buf = String::new();
        std::fs::File::read_to_string(&mut std::fs::File::open("puzzleData/day9.txt").unwrap(), &mut buf).unwrap();
        assert_eq!(458, calculate_risk_level(&buf));
    }
    #[test]
    fn part1_test() {
        assert_eq!(15, calculate_risk_level(TESTDATA));
    }

    #[test]
    fn part2_test() {
        let mut buf = String::new();
        std::fs::File::read_to_string(&mut std::fs::File::open("puzzleData/day9.txt").unwrap(), &mut buf).unwrap();
        assert_eq!(1391940, count_top_lake_tiles(&buf, 3));
    }

    #[test]
    fn part2() {
        assert_eq!(1134, count_top_lake_tiles(TESTDATA, 3));
    }

    const TESTDATA: &'static str = "2199943210
3987894921
9856789892
8767896789
9899965678";
}
