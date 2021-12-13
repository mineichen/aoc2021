use std::{io::Read, collections::HashSet};

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

fn count_dots(mut map: Vec<(i64, i64)>, instructions: &[FoldInstruction]) -> i64 {
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
    map.into_iter().collect::<HashSet<_>>().len() as i64
}

enum FoldInstruction {
    X(i64),
    Y(i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let (map, instructions) = parse(std::fs::File::open("puzzleData/day13.txt").unwrap()).unwrap();
        assert_eq!(706, count_dots(map, &instructions[0..1]));
    }

    #[test]
    fn part1_test() {
        let (map, instructions) = parse(std::io::Cursor::new(TEST_INPUT)).unwrap();
        assert_eq!(17, count_dots(map, &instructions[0..1]));
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