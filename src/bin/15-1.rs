use anyhow;
use aoc23;
use itertools::*;

fn hash(s: &str) -> u64 {
    assert!(s.is_ascii());
    s.trim()
        .bytes()
        .fold(0_u64, |acc, x| ((acc + x as u64) * 17) % 256)
}

fn main() -> anyhow::Result<()> {
    let mut lines = aoc23::read_input_lines();
    let sum: u64 = lines.next().unwrap().split(',').map(|s| hash(s)).sum();
    println!("{}", sum);
    Ok(())
}
