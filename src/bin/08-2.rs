use std::collections::{HashMap, HashSet};

use anyhow;
use aoc24::{self, Location};
use itertools::*;

fn main() -> anyhow::Result<()> {
    let lines = aoc24::read_input_lines().collect::<Vec<_>>();
    let max_line = lines.len() as isize;
    let max_col = lines.first().unwrap().chars().count() as isize;
    let mut location_map: HashMap<char, Vec<Location>> = HashMap::new();

    let antennas = lines
        .iter()
        .enumerate()
        .flat_map(|(l_nr, line)| {
            line.chars()
                .enumerate()
                .map(move |(c_nr, cha)| (l_nr, c_nr, cha))
        })
        .filter(|(l_nr, c_nr, cha)| *cha != '.');

    for (l_nr, c_nr, cha) in antennas {
        location_map
            .entry(cha)
            .or_default()
            .push(Location::new_usize(l_nr, c_nr));
    }

    let mut antinode_locations: HashSet<Location> = HashSet::new();

    let is_inside = |loc: &Location| {
        loc.is_inside_bounding_box(
            &Location::new(0, 0),
            &Location::new(max_line - 1, max_col - 1),
        )
    };

    for (_, ant_locations) in location_map.iter() {
        let pairs = ant_locations.iter().cartesian_product(ant_locations.iter());
        for (loc1, loc2) in pairs.filter(|(l1, l2)| l1 != l2) {
            let distance = loc2.distance(loc1);

            for i in 0.. {
                let new_antinode = loc1.apply_n_distance(&distance, -i);
                if is_inside(&new_antinode) {
                    antinode_locations.insert(new_antinode);
                } else {
                    break;
                }
            }
            for i in 0.. {
                let new_antinode = loc2.apply_n_distance(&distance, i);
                if is_inside(&new_antinode) {
                    antinode_locations.insert(new_antinode);
                } else {
                    break;
                }
            }
        }
    }
    println!("{}", antinode_locations.iter().count());

    Ok(())
}
