use anyhow;
use aoc24::{self};

fn main() -> anyhow::Result<()> {
    let lines = aoc24::read_input_lines();

    let (mut l1, mut l2) = lines
        .map(|line| {
            let mut parts = line.trim().split_whitespace();
            let n1: i32 = parts.next().unwrap().parse().unwrap();
            let n2: i32 = parts.next().unwrap().parse().unwrap();
            (n1, n2)
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();

    l1.sort();
    l2.sort();

    let res: u32 = l1.into_iter().zip(l2).map(|(n1, n2)| n1.abs_diff(n2)).sum();
    println!("{}", res);

    Ok(())
}
