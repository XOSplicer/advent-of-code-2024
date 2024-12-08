
use anyhow;
use aoc24::{self};

fn main() -> anyhow::Result<()> {
    let lines = aoc24::read_input_lines();

    let res = lines
        .filter(|line| {
            let report: Vec<i32> = line
                .trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let deltas: Vec<i32> = report
                .iter()
                .zip(report.iter().skip(1))
                .map(|(n1, n2)| n1 - n2)
                .collect();

            let asc = deltas.first().unwrap().signum();

            deltas
                .iter()
                .all(|delta| delta * asc >= 1 && delta * asc <= 3)
        })
        .count();

    println!("{}", res);

    Ok(())
}
