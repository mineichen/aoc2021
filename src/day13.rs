use std::{io::Read};

use simple_lines::ReadExt;

use crate::utils::AocError;


fn parse(r: impl Read) -> Result<(Vec<(i64, i64)>, Vec<FoldInstruction>), Box<dyn std::error::Error>> {
    let mut lines = r.lines_rc();
    let map: Result<Vec<_>, Box<dyn std::error::Error>> = lines
        .by_ref()
        .take_while(|x| match x {
            Ok(ok) => !ok.is_empty(),
            Err(_) => true
        }).map(|line| {
            let line = line?;
            let mut parts = line.split(',');
            let x = parts.next().ok_or(AocError::InvalidRowFormat)?.parse()?;
            let y = parts.next().ok_or(AocError::InvalidRowFormat)?.parse()?;
            Ok((x, y))
        }).collect();
    let map = map?;

    let instructions: Result<Vec<_>, Box<dyn std::error::Error>> = lines
        .map(|line| {
            let line = line?;
            let chars = &line["fold along ".len()..];
            let instruction = chars.chars().next().ok_or(AocError::InvalidRowFormat)?;
            let nr: i64 = chars[2..].parse()?;
            Ok(match instruction {
                'x' => FoldInstruction::X(nr),
                'y' => FoldInstruction::Y(nr),
                _ => return Err(AocError::InvalidRowFormat.into())
            })
        })
        .collect();

    Ok((map, instructions?))
}

fn apply_instructions(map: &mut Vec<(i64, i64)>, instructions: &[FoldInstruction]) {
    for instr in instructions {
        map.iter_mut().for_each(|(x, y)| {
            
            match instr {
                FoldInstruction::X(x_fold) => {
                    if *x > *x_fold {
                        *x = 2*x_fold - *x;
                    }
                },
                FoldInstruction::Y(y_fold) => {
                    if *y > *y_fold { 
                        *y = 2*y_fold - *y;
                    }
                }
            }
        });
    }
}

enum FoldInstruction {
    X(i64),
    Y(i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn part1() {
        let (mut map, instructions) = parse(std::fs::File::open("puzzleData/day13.txt").unwrap()).unwrap();
        apply_instructions(&mut map, &instructions[0..1]);
        assert_eq!(706, map.into_iter().collect::<HashSet<_>>().len() as i64);
    }

    #[test]
    fn part1_test() {
        let (mut map, instructions) = parse(std::io::Cursor::new(TEST_INPUT)).unwrap();
        apply_instructions(&mut map, &instructions[0..1]);
        assert_eq!(17, map.into_iter().collect::<HashSet<_>>().len() as i64);
    }

    #[test]
    fn part2() {
        let (mut map, instructions) = parse(std::fs::File::open("puzzleData/day13.txt").unwrap()).unwrap();
        apply_instructions(&mut map, &instructions);
        let (max_x, max_y) = map.iter().fold((0, 0), |(acc_x, acc_y), (x, y)| (acc_x.max(*x as usize), acc_y.max(*y as usize)));
        let (width, height) = (max_x + 2, max_y + 1);
        let mut coords = vec!(b'.'; width * height);
        for (x, y) in map {
            coords[x as usize + y as usize* width] = b'#';
        }
        for y in 1..height {
            coords[y * width - 1] = b'\n';
        }
        let out = std::str::from_utf8(coords.as_slice()).unwrap();
        assert_eq!("#....###..####...##.###....##.####.#..#\n#....#..#.#.......#.#..#....#.#....#..#\n#....#..#.###.....#.###.....#.###..####\n#....###..#.......#.#..#....#.#....#..#\n#....#.#..#....#..#.#..#.#..#.#....#..#\n####.#..#.#.....##..###...##..####.#..#.", out);
        // println!("{}", std::str::from_utf8(coords.as_slice()).unwrap());

    }
const TEST_INPUT : &'static str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
}