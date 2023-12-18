use std::collections::HashMap;

use anyhow;
use aoc23::{self, Direction, Location};
use itertools::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RGBColor(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Instruction {
    direction: Direction,
    steps: usize,
    color: RGBColor,
}

impl Instruction {
    fn from_line(line: &str) -> Self {
        let mut parts = line.trim().split_whitespace();
        let direction = match parts.next().unwrap() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            s => panic!("invalid direction: {}", s),
        };
        let steps: usize = parts.next().unwrap().parse().unwrap();
        let color_str = parts
            .next()
            .unwrap()
            .trim_start_matches("(#")
            .trim_end_matches(')');
        let color = RGBColor(u64::from_str_radix(color_str, 16).unwrap());
        Instruction {
            direction,
            steps,
            color,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Entry;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map(HashMap<Location, Entry>);

impl Map {
    fn new() -> Self {
        Map(HashMap::new())
    }

    fn max_row(&self) -> Option<isize> {
        self.0.keys().map(|loc| loc.row).max()
    }

    fn max_col(&self) -> Option<isize> {
        self.0.keys().map(|loc| loc.col).max()
    }

    fn min_row(&self) -> Option<isize> {
        self.0.keys().map(|loc| loc.row).min()
    }

    fn min_col(&self) -> Option<isize> {
        self.0.keys().map(|loc| loc.col).min()
    }

    fn rows(&self) -> usize {
        (self.max_row().unwrap_or(0) - self.min_row().unwrap_or(0)) as usize + 1
    }

    fn cols(&self) -> usize {
        (self.max_col().unwrap_or(0) - self.min_col().unwrap_or(0)) as usize + 1
    }

    fn total_area(&self) -> usize {
        self.rows() * self.cols()
    }

    fn filled_area(&self) -> usize {
        self.0.len()
    }

    fn unfilled_area(&self) -> usize {
        self.total_area() - self.filled_area()
    }

    fn flood_fill(&mut self, seed: Location, max_worklist: usize) {
        let mut worklist: Vec<Location> = Vec::new();

        let orig_map = self.clone();

        worklist.push(seed);
        while let Some(loc) = worklist.pop() {
            if worklist.len() > max_worklist {
                println!("warning: max_worklist reached, reducing flood_fill");
                continue;
            }
            if !orig_map.contains(loc) {
                continue;
            }
            match self.0.get(&loc).copied() {
                Some(Entry) => { /* already filled, continue */ }
                None => {
                    self.0.insert(loc, Entry);
                    worklist.push(loc.up());
                    worklist.push(loc.down());
                    worklist.push(loc.left());
                    worklist.push(loc.right());
                }
            }
        }
    }

    fn contains(&self, location: Location) -> bool {
        location.row <= self.max_row().unwrap_or(0)
            && location.row >= self.min_row().unwrap_or(0)
            && location.col <= self.max_col().unwrap_or(0)
            && location.col >= self.min_col().unwrap_or(0)
    }

    fn reserve(&mut self) {
        self.0.reserve(self.total_area() - self.0.len())
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();
    let instructions = lines.map(|s| Instruction::from_line(&s)).collect_vec();
    let mut map = Map::new();
    let mut curr_loc = Location::new(0, 0);
    map.0.insert(curr_loc, Entry);

    // mark trenches of instructions
    for instr in instructions {
        for _ in 0..instr.steps {
            curr_loc = curr_loc.apply(instr.direction);
            map.0.insert(curr_loc, Entry);
        }
    }

    println!("area: {}", map.total_area());

    let seeds = [
        Location::new(1, 1),
        Location::new(1, -1),
        Location::new(-1, 1),
        Location::new(-1, -1),
    ];

    for seed in seeds.into_iter().rev() {
        let mut m = map.clone();
        m.reserve();
        m.flood_fill(seed, m.total_area());
        println!(
            "seed {:?} => filled {}, unfilled {}",
            seed,
            m.filled_area(),
            m.unfilled_area()
        );
    }

    // alternate approach: sum up triangle area of each step with the previous

    Ok(())
}
