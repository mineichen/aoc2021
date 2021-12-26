use itertools::Itertools;
use pathfinding::{
    directed::astar::astar,
    prelude::{absdiff, Matrix},
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (absdiff(self.0, other.0) + absdiff(self.1, other.1)) as u32
    }

    fn successors(&self, m: &Matrix<u8>) -> Vec<(Pos, u32)> {
        m.neighbours((self.0, self.1), false)
            .map(|(x, y)| (Pos(x, y), m[(x, y)] as u32))
            .collect()
    }
}

fn calculate_shortest_path(m: &Matrix<u8>) -> u32 {
    let goal = Pos(m.columns - 1, m.rows - 1);
    dbg!(&goal);
    let result = astar(
        &Pos(0, 0),
        |p| p.successors(m),
        |p| p.distance(&goal),
        |p| *p == goal,
    );
    let (_, dist) = result.unwrap();
    dist
}

const EXPAND: u8 = 5;
fn wrap_value(i: u8, offset: u8) -> u8 {
    match i + offset {
        i if i < 10 => i,
        i => i - 10 + 1,
    }
}
fn read_part2_matrix(input: &str) -> Matrix<u8> {
    let mut vec = input
        .lines()
        .map(|c| {
            let mut vec = Vec::with_capacity(c.bytes().len() * EXPAND as usize);
            for i in 0..EXPAND {
                vec.extend(c.bytes().map(|x| wrap_value(x - b'0', i)))
            }
            vec
        })
        .collect_vec();
    let height = vec.len();
    for i in 1..EXPAND {
        for inner in 0..height {
            vec.push(vec[inner].iter().map(|x| wrap_value(*x, i)).collect_vec());
        }
    }

    vec.into_iter().collect()
}

fn print_matrix(m: &Matrix<u8>) {
    for y in 0..m.rows {
        for x in 0..m.columns {
            print!("{}", m[(y, x)]);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1() {
        let buf = std::fs::read_to_string("puzzleData/day15.txt").unwrap();
        let m: Matrix<u8> = buf.lines().map(|c| c.bytes().map(|x| x - b'0')).collect();
        assert_eq!(527, calculate_shortest_path(&m));
    }
    #[test]
    fn part1_test() {
        let m: Matrix<u8> = TESTDATA
            .lines()
            .map(|c| c.bytes().map(|x| (x - b'0')))
            .collect();
        assert_eq!(40, calculate_shortest_path(&m));
    }

    #[test]
    fn part2_input() {
        let buf = std::fs::read_to_string("puzzleData/day15.txt").unwrap();
        let mat = read_part2_matrix(&buf);
        assert_eq!(2887, calculate_shortest_path(&mat));
    }

    #[test]
    fn part2_test() {
        let mat = read_part2_matrix(&TESTDATA);
        assert_eq!(315, calculate_shortest_path(&mat));
    }

    #[test]
    fn wrap_value_10() {
        assert_eq!(2, super::wrap_value(9, 2));
    }
    const TESTDATA: &'static str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
}
