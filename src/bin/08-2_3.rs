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

#[derive(Debug, Clone)]
struct Map {
    instructions: Box<[Instr]>,
    map: HashMap<String, MapEntry>,
}

impl Map {
    fn from_lines(mut lines: impl Iterator<Item = String>) -> Self {
        let instructions = lines
            .next()
            .unwrap()
            .trim()
            .chars()
            .map(Instr::from_char)
            .collect_vec()
            .into_boxed_slice();
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
        Map { instructions, map }
    }

    fn start_nodes(&self) -> Box<[&str]> {
        self.map
            .keys()
            .filter(|s| s.ends_with('A'))
            .map(|s| s.as_str())
            .collect_vec()
            .into_boxed_slice()
    }
    fn end_nodes(&self) -> Box<[&str]> {
        self.map
            .keys()
            .filter(|s| s.ends_with('Z'))
            .map(|s| s.as_str())
            .collect_vec()
            .into_boxed_slice()
    }

    fn find_some_end_node(&self, from_node: &str) -> (&str, u64) {
        let mut steps = 0_u64;
        let mut iter = self.instructions.iter().cycle();

        // do at least one step

        let mut instr = iter.next().unwrap();
        let mut node = self.map.get(from_node).unwrap().select(*instr);
        steps += 1;

        while !node.ends_with('Z') {
            instr = iter.next().unwrap();
            node = self.map.get(node).unwrap().select(*instr);
            steps += 1;
        }

        (node, steps)
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();
    let map = Map::from_lines(lines);

    let start_nodes = map.start_nodes();
    let end_nodes = map.end_nodes();

    let first_end_nodes: HashMap<&str, (&str, u64)> = start_nodes
        .iter()
        .map(|&s| (s, map.find_some_end_node(s)))
        .collect();

    let end_to_end: HashMap<&str, (&str, u64)> = end_nodes
        .iter()
        .map(|&s| (s, map.find_some_end_node(s)))
        .collect();

    assert!(
        end_to_end.iter().all(|(k, (s, _))| k == s),
        "End elements cycle through multple ends, cant calc"
    );

    let end_to_self_cycle_steps: HashMap<&str, u64> = end_to_end
        .clone()
        .into_iter()
        .map(|(k, (_, s))| (k, s))
        .collect();

    assert!(first_end_nodes
        .iter()
        .all(|(_, (end, start_steps))| start_steps == end_to_self_cycle_steps.get(end).unwrap()));

    println!("{:?}", &first_end_nodes);
    println!("{:?}", &end_to_end);

    // lcm of all path steps

    let solution = end_to_self_cycle_steps
        .values()
        .copied()
        .reduce(|acc, steps| num::integer::lcm(acc, steps))
        .unwrap();
    println!("{}", solution);
    Ok(())
}
