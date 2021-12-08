fn parse(input: &str) -> Result<Simulation, Box<dyn std::error::Error>> {
    let r: Result<Vec<u8>, Box<dyn std::error::Error>> =
        input.split(',').map(|e| Ok(e.parse()?)).collect();
    Ok(Simulation::new(r?))
}

struct Simulation {
    fishes: [u64; 9],
}

impl Simulation {
    fn new(state: impl IntoIterator<Item = u8>) -> Self {
        let mut fishes = [0u64; 9];

        for time in state.into_iter() {
            fishes[time as usize] += 1;
        }
        Self { fishes }
    }
    fn step(&mut self) {
        self.fishes.rotate_left(1);
        self.fishes[6] += self.fishes[8];
    }

    fn steps(&mut self, n: u32) -> u64 {
        for x in 0..n {
            self.step();
        }
        self.fishes.iter().sum::<u64>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::process_results;

    const TEST_INPUT: &'static str = "3,4,3,1,2";
    const REAL_INPUT: &'static str = "3,1,4,2,1,1,1,1,1,1,1,4,1,4,1,2,1,1,2,1,3,4,5,1,1,4,1,3,3,1,1,1,1,3,3,1,3,3,1,5,5,1,1,3,1,1,2,1,1,1,3,1,4,3,2,1,4,3,3,1,1,1,1,5,1,4,1,1,1,4,1,4,4,1,5,1,1,4,5,1,1,2,1,1,1,4,1,2,1,1,1,1,1,1,5,1,3,1,1,4,4,1,1,5,1,2,1,1,1,1,5,1,3,1,1,1,2,2,1,4,1,3,1,4,1,2,1,1,1,1,1,3,2,5,4,4,1,3,2,1,4,1,3,1,1,1,2,1,1,5,1,2,1,1,1,2,1,4,3,1,1,1,4,1,1,1,1,1,2,2,1,1,5,1,1,3,1,2,5,5,1,4,1,1,1,1,1,2,1,1,1,1,4,5,1,1,1,1,1,1,1,1,1,3,4,4,1,1,4,1,3,4,1,5,4,2,5,1,2,1,1,1,1,1,1,4,3,2,1,1,3,2,5,2,5,5,1,3,1,2,1,1,1,1,1,1,1,1,1,3,1,1,1,3,1,4,1,4,2,1,3,4,1,1,1,2,3,1,1,1,4,1,2,5,1,2,1,5,1,1,2,1,2,1,1,1,1,4,3,4,1,5,5,4,1,1,5,2,1,3";
    #[test]
    fn part1_test() {
        let mut game = parse(TEST_INPUT).unwrap();
        assert_eq!(26, game.steps(18));
    }
    #[test]
    fn part1() {
        let mut game = parse(REAL_INPUT).unwrap();
        assert_eq!(1710166656900, game.steps(256));
    }
}
