use std::collections::{HashMap, HashSet};

use anyhow;
use aoc24::{self, Location};
use itertools::*;

fn main() -> anyhow::Result<()> {
    let lines = aoc24::read_input_lines().collect::<Vec<_>>();
    let max_line = lines.len() as isize;
    let max_col = lines[1].chars().count() as isize;
    let mut location_map: HashMap<char, Vec<Location>> = HashMap::new();
    for (l_nr, line) in lines.iter().enumerate() {
        for (c_nr, cha) in line.chars().enumerate() {
            if cha != '.' {
                location_map
                    .entry(cha)
                    .or_default()
                    .push(Location::new_usize(l_nr, c_nr));
            }
        }
    }

    let mut antinode_locations: HashSet<Location> = HashSet::new();

    for (_ant_char, ant_locations) in location_map.iter() {
        let pairs = ant_locations.iter().cartesian_product(ant_locations.iter());
        for (loc1, loc2) in pairs.filter(|(l1, l2)| l1 != l2) {
            let distance = loc2.distance(loc1);
            antinode_locations.insert(loc1.apply_n_distance(&distance, -1));
            antinode_locations.insert(loc1.apply_n_distance(&distance, 2));
        }
    }
    println!(
        "{}",
        antinode_locations
            .iter()
            .filter(|loc| loc.row >= 0 && loc.row < max_line && loc.col >= 0 && loc.col < max_col)
            .count()
    );

    Ok(())
}
