use std::collections::{HashMap, HashSet};

use anyhow;
use aoc23;
use aoc23::{Direction, Location};
use itertools::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entry {
    Start,
    GardenPlot,
    Rock,
}

fn parse(lines: impl Iterator<Item = String>) -> HashMap<Location, Entry> {
    let pattern: HashMap<Location, Entry> = lines
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    let loc = Location::new(row as isize, col as isize);
                    let entry = match c {
                        '.' => Entry::GardenPlot,
                        '#' => Entry::Rock,
                        'S' => Entry::Start,
                        _ => panic!("Invalid entry {}", c),
                    };
                    (loc, entry)
                })
                .collect_vec()
        })
        .collect();
    pattern
}

fn next_reachable(
    garden_plots: &HashSet<Location>,
    current: HashSet<Location>,
) -> HashSet<Location> {
    use Direction::*;
    current
        .into_iter()
        .flat_map(|loc| {
            [Up, Down, Left, Right]
                .into_iter()
                .map(move |dir| loc.apply(dir))
                .filter(|new_loc| garden_plots.contains(new_loc))
        })
        .collect()
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();
    let pattern = parse(lines);
    let start = pattern
        .iter()
        .find_map(|(&loc, &e)| (e == Entry::Start).then_some(loc))
        .unwrap();
    let garden_plots: HashSet<Location> = pattern
        .iter()
        .filter_map(|(&loc, &e)| (e != Entry::Rock).then_some(loc))
        .collect();

    let mut reachable = HashSet::new();
    reachable.insert(start);
    for _ in 0..64 {
        reachable = next_reachable(&garden_plots, reachable);
    }

    let sum: usize = reachable.len();
    println!("{}", sum);
    Ok(())
}
