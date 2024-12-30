use anyhow;
use aoc24::{self};
use itertools::*;

fn main() -> anyhow::Result<()> {
    let lines = aoc24::read_input_lines();

    let (l1, l2) = lines
        .map(|line| {
            let mut parts = line.trim().split_whitespace();
            let n1: i32 = parts.next().unwrap().parse().unwrap();
            let n2: i32 = parts.next().unwrap().parse().unwrap();
            (n1, n2)
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();

    let right_counts = l2.into_iter().counts();

    let res: u64 = l1
        .into_iter()
        .map(|n1| n1 as u64 * *right_counts.get(&n1).unwrap_or(&0) as u64)
        .sum();

    println!("{}", res);

    Ok(())
}
