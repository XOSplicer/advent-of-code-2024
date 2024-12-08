use anyhow;
use aoc24::{self};
use itertools::Itertools;

fn possible_results(list: &[u64]) -> Vec<u64> {
    match list {
        [] => Vec::new(),
        [n] => vec![*n],
        [ns @ .., n] => {
            let possible_res = possible_results(ns);
            possible_res
                .iter()
                .map(|r| n * r)
                .chain(possible_res.iter().map(|r| n + r))
                .collect_vec()
        }
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc24::read_input_lines();

    let mut res: u64 = 0;
    for line in lines {
        let mut parts = line.split(':');
        let expected: u64 = parts.next().unwrap().parse().unwrap();
        let nums: Vec<u64> = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if possible_results(&nums).contains(&expected) {
            res += expected;
        }
    }

    println!("{}", res);

    Ok(())
}
