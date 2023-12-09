use anyhow;
use aoc23;
use itertools::*;

fn main() -> anyhow::Result<()> {
    let mut lines = aoc23::read_input_lines();
    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_vec();
    let distances: Vec<u64> = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_vec();

    let win_combinations: Vec<u64> = times
        .iter()
        .copied()
        .zip(distances.iter().copied())
        .map(|(time, distance)| (0..=time).filter(|t| t * (time - t) > distance).count() as u64)
        .collect_vec();

    let margin_of_error: u64 = win_combinations.iter().product();
    println!("{}", margin_of_error);
    Ok(())
}
