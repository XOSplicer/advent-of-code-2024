use anyhow;
use aoc24::{self};
use itertools::Itertools;

fn op_add(r: u64, n: u64) -> u64 {
    r + n
}
fn op_mul(r: u64, n: u64) -> u64 {
    r * n
}
fn op_concat(r: u64, n: u64) -> u64 {
    let m = 10_u64.pow((n as f64).log10().floor() as u32 + 1);
    r * m + n
}

fn possible_results(list: &[u64]) -> Vec<u64> {
    match list {
        [] => Vec::new(),
        [n] => vec![*n],
        [ns @ .., n] => {
            let possible_res = possible_results(ns);
            possible_res
                .iter()
                .map(|r| op_add(*r, *n))
                .chain(possible_res.iter().map(|r| op_mul(*r, *n)))
                .chain(possible_res.iter().map(|r| op_concat(*r, *n)))
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
