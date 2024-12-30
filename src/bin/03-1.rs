use anyhow;
use aoc24::{self};
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let input = aoc24::read_input_file();
    // matches mul(123,4) and captures 123 and 4
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let res: u64 = re
        .captures_iter(&input)
        .map(|cap| {
            let (_, [s1, s2]) = cap.extract();
            let n1: u64 = s1.parse().unwrap();
            let n2: u64 = s2.parse().unwrap();
            n1 * n2
        })
        .sum();
    println!("{}", res);
    Ok(())
}
