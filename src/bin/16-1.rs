use std::collections::{HashMap, HashSet};

use anyhow;
use aoc23;
use aoc23::{Direction, Location};
use itertools::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum EntryKind {
    /// .
    Empty,
    /// \
    LeftMirror,
    /// /
    RightMirror,
    /// |
    VerticalSplitter,
    /// -
    HorizontalSplitter,
}

impl EntryKind {
    fn from_char(c: char) -> Self {
        match c {
            '.' => EntryKind::Empty,
            '\\' => EntryKind::LeftMirror,
            '/' => EntryKind::RightMirror,
            '|' => EntryKind::VerticalSplitter,
            '-' => EntryKind::HorizontalSplitter,
            _ => panic!("invalid entry: {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PatternEntry {
    location: Location,
    kind: EntryKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pattern {
    inner: HashMap<Location, PatternEntry>,
    rows: isize,
    cols: isize,
}

impl Pattern {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let inner: HashMap<Location, PatternEntry> = lines
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(col, c)| PatternEntry {
                        kind: EntryKind::from_char(c),
                        location: Location::new(row as isize, col as isize),
                    })
                    .map(|entry| (entry.location, entry))
                    .collect_vec()
            })
            .collect();
        let rows = inner.keys().map(|loc| loc.row).max().unwrap() + 1;
        let cols = inner.keys().map(|loc| loc.col).max().unwrap() + 1;
        Pattern { inner, rows, cols }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct LightStep {
    location: Location,
    direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StepResult {
    next: Vec<LightStep>,
}

impl EntryKind {
    fn apply_direction(&self, dir: Direction) -> Vec<Direction> {
        use Direction::*;
        use EntryKind::*;
        match (self, dir) {
            (Empty, _) => vec![dir],
            (LeftMirror, Up) => vec![Left],
            (LeftMirror, Down) => vec![Right],
            (LeftMirror, Left) => vec![Up],
            (LeftMirror, Right) => vec![Down],
            (RightMirror, Up) => vec![Right],
            (RightMirror, Down) => vec![Left],
            (RightMirror, Left) => vec![Down],
            (RightMirror, Right) => vec![Up],
            (VerticalSplitter, Up | Down) => vec![dir],
            (VerticalSplitter, Left | Right) => vec![Up, Down],
            (HorizontalSplitter, Left | Right) => vec![dir],
            (HorizontalSplitter, Up | Down) => vec![Left, Right],
        }
    }
}

impl Pattern {
    fn step(&self, step: LightStep) -> StepResult {
        let location = step.location.apply(step.direction);
        if let Some(entry) = self.inner.get(&location) {
            let dirs = entry.kind.apply_direction(step.direction);
            StepResult {
                next: dirs
                    .into_iter()
                    .map(|direction| LightStep {
                        location,
                        direction,
                    })
                    .collect_vec(),
            }
        } else {
            StepResult { next: Vec::new() }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();
    let pattern = Pattern::from_lines(lines);

    let mut seen_light_steps: HashSet<LightStep> = HashSet::new();
    let mut energized_tiles: HashSet<Location> = HashSet::new();
    let mut worklist: Vec<LightStep> = Vec::new();
    worklist.push(LightStep {
        location: Location::new(0, -1),
        direction: Direction::Right,
    });

    while let Some(step) = worklist.pop() {
        if seen_light_steps.contains(&step) {
            continue;
        }

        //        dbg!(step);
        seen_light_steps.insert(step);
        energized_tiles.insert(step.location);
        let res = pattern.step(step);
        worklist.extend(res.next);
    }

    // ignore (0, -1) step
    let count: usize = energized_tiles.len() - 1;
    println!("{}", count);
    Ok(())
}
