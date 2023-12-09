use std::collections::HashMap;

use anyhow;
use aoc23;
use itertools::*;

#[derive(Debug, Clone, Copy)]
enum Instr {
    Left,
    Right,
}

impl Instr {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Instr::Left,
            'R' => Instr::Right,
            _ => panic!("Unknown Instr {}", c),
        }
    }
}

#[derive(Debug, Clone)]
struct MapEntry {
    left: String,
    right: String,
}

impl MapEntry {
    fn from_str(s: &str) -> Self {
        let mut parts = s
            .trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(',');
        MapEntry {
            left: parts.next().unwrap().trim().to_string(),
            right: parts.next().unwrap().trim().to_string(),
        }
    }
    fn select(&self, instr: Instr) -> &str {
        match instr {
            Instr::Left => self.left.as_str(),
            Instr::Right => self.right.as_str(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut lines = aoc23::read_input_lines();
    let instructions = lines
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(Instr::from_char)
        .collect_vec();
    lines.next(); // empty
    let map: HashMap<String, MapEntry> = lines
        .map(|line| {
            let mut parts = line.trim().split('=');
            (
                parts.next().unwrap().trim().to_string(),
                MapEntry::from_str(parts.next().unwrap().trim()),
            )
        })
        .collect();

    let mut node = "AAA";
    let mut steps = 0;
    let mut iter = instructions.iter().cycle();
    while node != "ZZZ" {
        let instr = iter.next().unwrap();
        node = map.get(node).unwrap().select(instr.clone());
        steps += 1;
    }
    println!("{}", steps);
    Ok(())
}
