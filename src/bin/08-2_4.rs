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

    let _end_to_self_cycle_steps: HashMap<&str, u64> = end_to_end
        .clone()
        .into_iter()
        .map(|(k, (_, s))| (k, s))
        .collect();

    println!("{:?}", &first_end_nodes);
    println!("{:?}", &end_to_end);

    // solve
    // O1+n1*C1 =  O2+n2*C2 = O3+n3*C3
    // for n1 ... nn
    // with initial offset 0n and Cycle Cn

    todo!();

    /*
    for i in 1..1000_u64 {
        // v is (n1, n2, ... nn) with each nj < i
        println!("iteration: {}", i);
        for v in start_nodes.iter().map(|_| (0..i)).multi_cartesian_product() {
            if let Ok(solution) = first_end_nodes
                .iter()
                .zip(v)
                .map(|((&start_node, (end_node, initial_steps)), vn)| {
                    initial_steps + vn * end_to_self_cycle_steps.get(end_node).unwrap()
                })
                .all_equal_value()
            {
                println!("solution: {}", solution);
                return Ok(());
            }
        }
    }
    */

    //println!("{}", steps);
    Ok(())
}
