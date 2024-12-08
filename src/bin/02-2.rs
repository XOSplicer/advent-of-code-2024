use std::collections::{HashMap, HashSet};

use anyhow;
use aoc24::{self, Location};
use itertools::*;

fn original_save(report: &[i32]) -> bool {
    let deltas: Vec<i32> = report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(n1, n2)| n1 - n2)
        .collect();

    let asc = deltas.first().unwrap().signum();

    deltas
        .iter()
        .all(|delta| delta * asc >= 1 && delta * asc <= 3)
}

fn tolerated_save(report: &[i32]) -> bool {
    for i in 0..report.len() {
        let fixed_report = report
            .iter()
            .enumerate()
            .filter(|(idx, n)| *idx != i)
            .map(|(_, n)| *n)
            .collect_vec();

        let deltas: Vec<i32> = fixed_report
            .iter()
            .zip(fixed_report.iter().skip(1))
            .map(|(n1, n2)| n1 - n2)
            .collect();

        let asc = deltas.first().unwrap().signum();

        let valid = deltas
            .iter()
            .all(|delta| delta * asc >= 1 && delta * asc <= 3);
        if valid {
            return valid;
        }
    }
    false
}

fn main() -> anyhow::Result<()> {
    let lines = aoc24::read_input_lines();

    let res = lines
        .filter(|line| {
            let report: Vec<i32> = line
                .trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            original_save(&report) || tolerated_save(&report)
        })
        .count();

    println!("{}", res);

    Ok(())
}
