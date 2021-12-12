use itertools::process_results;
use simple_lines::ReadExt;
use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    io::Read,
    rc::Rc,
};

use crate::utils::AocError;

type CaveMap = HashMap<Rc<str>, CaveReferences>;

fn parse(m: impl Read) -> Result<CaveMap, Box<dyn std::error::Error>> {
    process_results(
        m.lines_rc().map(|f| {
            let f = f?;
            let mut parts = f.split('-');
            let a: Cave = parts.next().ok_or(AocError::InvalidRowFormat)?.try_into()?;
            let b: Cave = parts.next().ok_or(AocError::InvalidRowFormat)?.try_into()?;
            Ok((a, b))
        }),
        |iter| {
            let mut result: HashMap<Rc<str>, CaveReferences> = HashMap::new();
            for (a, b) in iter {
                let a_id = a.id.clone();
                let b_id = b.id.clone();

                let a_entry = result.entry(a_id).or_default();
                match b.t {
                    CaveType::Big => a_entry.big.push(b.id),
                    CaveType::Small => a_entry.small.push(b.id),
                };

                let b_entry = result.entry(b_id).or_default();
                match a.t {
                    CaveType::Big => b_entry.big.push(a.id),
                    CaveType::Small => b_entry.small.push(a.id),
                };
            }
            result
        },
    )
}

impl<'a> TryFrom<&'a str> for Cave {
    type Error = AocError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if let Some(first_char) = value.chars().next() {
            Ok(Cave {
                id: Rc::from(value),
                t: first_char
                    .is_uppercase()
                    .then(|| CaveType::Big)
                    .unwrap_or(CaveType::Small),
            })
        } else {
            Err(AocError::InvalidRowFormat)
        }
    }
}

#[derive(Debug, Clone)]
struct Cave {
    id: Rc<str>,
    t: CaveType,
}

#[derive(Debug, Clone)]
enum CaveType {
    Big,
    Small,
}

#[derive(Debug, Default)]
struct CaveReferences {
    big: Vec<Rc<str>>,
    small: Vec<Rc<str>>,
}

fn count_paths(map: CaveMap, allow_single_double_visit: bool) -> u64 {
    let mut queue = vec![(
        Rc::<str>::from("start"),
        HashSet::<Rc<str>>::new(),
        allow_single_double_visit,
    )];
    let mut solutions = 0;
    while let Some((cur_id, visited, allow_double)) = queue.pop() {
        let cur = &map[&cur_id];
        for small in cur.small.iter() {
            let borrow: &str = small.borrow();

            if borrow == "end" {
                //println!("{:?}", visited);
                solutions += 1;
            } else {
                let visit_already = visited.contains(small);
                if !visit_already || allow_double && borrow != "start" {
                    let mut clone = visited.clone();
                    clone.insert(cur_id.clone());
                    queue.push((Rc::clone(small), clone, allow_double && !visit_already));
                }
            }
        }
        for big in cur.big.iter() {
            let mut clone = visited.clone();
            clone.insert(cur_id.clone());
            queue.push((Rc::clone(big), clone, allow_double));
        }
    }
    solutions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let map = parse(std::fs::File::open("puzzleData/day12.txt").unwrap()).unwrap();
        assert_eq!(4186, count_paths(map, false));
    }
    #[test]
    fn part1_test() {
        let map = parse(std::io::Cursor::new(TESTDATA)).unwrap();
        assert_eq!(19, count_paths(map, false));
    }

    #[test]
    fn part2_real() {
        let map = parse(std::fs::File::open("puzzleData/day12.txt").unwrap()).unwrap();
        assert_eq!(92111, count_paths(map, true));
    }

    #[test]
    fn part2_test() {
        let map = parse(std::io::Cursor::new(TESTDATA)).unwrap();
        assert_eq!(103, count_paths(map, true));
    }

    const TESTDATA: &'static str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
}
