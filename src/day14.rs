use itertools::{Itertools, MinMaxResult};
use simple_lines::ReadExt;
use std::{collections::HashMap, io::Read};

use crate::{utils::AocError, DynError};
type ParseResult = Result<(Vec<ListItem>, Vec<((u8, u8), u8)>, u8), DynError>;

fn parse(input: impl Read) -> ParseResult {
    let mut known = HashMap::new();
    let mut lines = input.lines_rc();
    let initial = lines
        .next()
        .ok_or(AocError::EmptyInput)??
        .bytes()
        .enumerate()
        .map(|(index, value)| {
            let len = known.len();
            let value = *known.entry(value).or_insert(len as u8);
            ListItem {
                value,
                next: index + 1,
            }
        })
        .collect();

    let rules = lines
        .skip(1)
        .map(|line| {
            let line = line?;
            let mut chars = line.bytes();
            let before = chars.next().ok_or(AocError::InvalidRowFormat)?;
            let after = chars.next().ok_or(AocError::InvalidRowFormat)?;
            let insert = chars.nth(4).ok_or(AocError::InvalidRowFormat)?;

            let len = known.len() as u8;
            let before = *known.entry(before).or_insert(len);
            let len = known.len() as u8;
            let after = *known.entry(after).or_insert(len);
            let len = known.len() as u8;
            let insert = *known.entry(insert).or_insert(len);

            Ok(((before, after), insert))
        })
        .collect::<Result<Vec<_>, DynError>>()?;
    Ok((initial, rules, known.len() as u8))
}
// :
#[derive(Clone, Copy, Debug)]
struct ListItem {
    value: u8,
    next: usize,
}

fn try_apply_rules(
    values: &mut Vec<ListItem>,
    rules: impl Iterator<Item = Result<((u8, u8), u8), DynError>>,
) -> Result<(), DynError> {
    let rules_map: Result<HashMap<_, _>, Box<dyn std::error::Error>> = rules.collect();
    apply_rules(values, &rules_map?);
    Ok(())
}

fn apply_rules(value: &mut Vec<ListItem>, rules: &HashMap<(u8, u8), u8>) {
    let mut current = (0, value[0]);
    for _ in 0..value.len() - 1 {
        let next = value[current.1.next];
        if let Some(insert) = rules.get(&(current.1.value, next.value)) {
            value.push(ListItem {
                value: *insert,
                next: current.1.next,
            });
            value[current.0].next = value.len() - 1;
        }
        current = (current.1.next, next);
    }
}

fn apply_n_rules_fast(
    value: Vec<ListItem>,
    rules: Vec<((u8, u8), u8)>,
    n: usize,
    unique: u8,
) -> MinMaxResult<u64> {
    let unique = unique as usize;
    let unique_square = unique * unique;
    let mut actual = vec![0u64; unique_square];
    let last = match value.last() {
        Some(x) => x.value as usize,
        None => return MinMaxResult::NoElements,
    };

    let pairs = value
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| (a.value, b.value));
    for (before, after) in pairs {
        actual[(before as usize * unique + after as usize)] += 1;
    }

    let mut rules_matrix = vec![u8::MAX; unique_square];
    for ((before, after), between) in rules {
        rules_matrix[(before as usize * unique + after as usize)] = between;
    }

    let mut buffer = vec![0u64; unique_square];
    for _ in 0..n {
        for (index, (&rule, &count)) in rules_matrix.iter().zip(actual.iter()).enumerate() {
            if rule == u8::MAX {
                buffer[index] += count;
            } else {
                let cur_before = index / unique;
                let cur_after = index % unique;
                let rule = rule as usize;
                buffer[cur_before * unique + rule] += count;
                buffer[rule * unique + cur_after] += count;
            }
        }
        std::mem::swap(&mut actual, &mut buffer);
        buffer.clear();
        buffer.resize(unique_square, 0);
    }
    // Add last item to any field in it's row
    actual[last * unique] += 1;
    actual
        .chunks(unique)
        .map(|x| x.iter().sum::<u64>())
        .minmax()
}

fn get_min_max_occurance(items: Vec<ListItem>) -> MinMaxResult<u64> {
    let mut lut = HashMap::<u8, u64, _>::new();
    for item in items {
        *lut.entry(item.value).or_default() += 1;
    }
    lut.into_iter().map(|(_, value)| value).minmax()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_test() {
        let (mut init, rules, _) = parse(std::io::Cursor::new(TESTDATA)).unwrap();
        let rules_map: HashMap<_, _> = rules.into_iter().collect();
        for _ in 0..10 {
            apply_rules(&mut init, &rules_map);
        }
        if let MinMaxResult::MinMax(min, max) = get_min_max_occurance(init) {
            assert_eq!(max - min, 1588);
        } else {
            panic!("Expected list with at least four elements");
        }
    }

    #[test]
    fn part1() {
        let (mut init, rules, _) =
            parse(std::fs::File::open("puzzleData/day14.txt").unwrap()).unwrap();
        let rules_map: HashMap<_, _> = rules.into_iter().collect();
        for _ in 0..10 {
            apply_rules(&mut init, &rules_map);
        }
        if let MinMaxResult::MinMax(min, max) = get_min_max_occurance(init) {
            assert_eq!(max - min, 2068);
        } else {
            panic!("Expected list with at least four elements");
        }
    }

    #[test]
    fn part2() {
        let (init, rules, unique) =
            parse(std::fs::File::open("puzzleData/day14.txt").unwrap()).unwrap();
        let result = apply_n_rules_fast(init, rules, 40, unique);
        if let MinMaxResult::MinMax(min, max) = result {
            assert_eq!(max - min, 2_158_894_777_814);
        } else {
            panic!("Expected list with at least four elements");
        }
    }

    const TESTDATA: &'static str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
}
