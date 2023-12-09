use anyhow;
use aoc23;
use itertools::*;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Seq(Vec<i64>);

impl Seq {
    fn from_line(line: &str) -> Self {
        let inner = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect_vec();
        Seq(inner)
    }

    fn extrapolate(&self) -> i64 {
        match self.stable_value() {
            Some(value) => value,
            None => self.0.last().unwrap() + self.sub_seq().extrapolate(),
        }
    }

    fn sub_seq(&self) -> Seq {
        let inner = self
            .0
            .iter()
            .copied()
            .tuple_windows()
            .map(|(x, y)| y - x)
            .collect_vec();
        Seq(inner)
    }

    fn stable_value(&self) -> Option<i64> {
        self.0.iter().all_equal_value().copied().ok()
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();
    let seqs = lines.map(|s| Seq::from_line(&s)).collect_vec();

    let sum: i64 = seqs.par_iter().map(|seq| seq.extrapolate()).sum();
    println!("{}", sum);
    Ok(())
}
