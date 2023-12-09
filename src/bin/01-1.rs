use anyhow;
use aoc23;
use itertools::*;

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();
    let sum: u32 = lines
        .map(|s| {
            let digits = s
                .chars()
                .filter(|c| c.is_numeric())
                .filter_map(|c| c.to_digit(10))
                .collect_vec();
            digits.first().copied().unwrap_or(0) * 10 + digits.last().copied().unwrap_or(0)
        })
        .sum();
    println!("{}", sum);
    Ok(())
}
