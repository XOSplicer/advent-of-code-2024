use std::{
    collections::{BTreeMap, BinaryHeap, HashMap, HashSet},
    iter::once,
};

use anyhow;
use aoc24::{self, read_visual_map, Direction, Distance, Location};
use itertools::Itertools;
use rayon::prelude::*;

struct Kernel3Input<'a, T> {
    /// row-major kernel view
    inner: [[Option<&'a T>; 3]; 3],
}

fn apply_kernel3_par<'a, T, F, U>(
    input: &'a BTreeMap<Location, T>,
    kernel: F,
) -> BTreeMap<Location, U>
where
    T: Send + Sync,
    U: Send + Sync,
    F: Fn(Kernel3Input<'a, T>) -> Option<U> + Send + Sync,
{
    input
        .par_iter()
        .filter_map(|(loc, value)| {
            let kernel_input = Kernel3Input {
                inner: [
                    [
                        input.get(&loc.apply_distance(&Distance::new(-1, -1))),
                        input.get(&loc.apply_distance(&Distance::new(-1, 0))),
                        input.get(&loc.apply_distance(&Distance::new(-1, 1))),
                    ],
                    [
                        input.get(&loc.apply_distance(&Distance::new(0, -1))),
                        Some(value),
                        input.get(&loc.apply_distance(&Distance::new(0, 1))),
                    ],
                    [
                        input.get(&loc.apply_distance(&Distance::new(1, -1))),
                        input.get(&loc.apply_distance(&Distance::new(1, 0))),
                        input.get(&loc.apply_distance(&Distance::new(1, 1))),
                    ],
                ],
            };
            let new_value = kernel(kernel_input);
            new_value.map(|v| (*loc, v))
        })
        .collect()
}

fn left_edge_kernel(input: Kernel3Input<char>) -> Option<char> {
    match (input.inner[1][0], input.inner[1][1]) {
        (None, None) => None,
        (None, Some(c)) => Some(*c),
        (Some(_), None) => None,
        (Some(o), Some(c)) => (o != c).then_some(*c),
    }
}

fn right_edge_kernel(input: Kernel3Input<char>) -> Option<char> {
    match (input.inner[1][2], input.inner[1][1]) {
        (None, None) => None,
        (None, Some(c)) => Some(*c),
        (Some(_), None) => None,
        (Some(o), Some(c)) => (o != c).then_some(*c),
    }
}

fn upper_edge_kernel(input: Kernel3Input<char>) -> Option<char> {
    match (input.inner[0][1], input.inner[1][1]) {
        (None, None) => None,
        (None, Some(c)) => Some(*c),
        (Some(_), None) => None,
        (Some(o), Some(c)) => (o != c).then_some(*c),
    }
}

fn lower_edge_kernel(input: Kernel3Input<char>) -> Option<char> {
    match (input.inner[2][1], input.inner[1][1]) {
        (None, None) => None,
        (None, Some(c)) => Some(*c),
        (Some(_), None) => None,
        (Some(o), Some(c)) => (o != c).then_some(*c),
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc24::read_input_lines();
    let field = aoc24::read_visual_map(lines);

    // use floodfill to generate ids of connected locations

    let mut connected_locs: Vec<HashSet<Location>> = Vec::new();
    for (loc, c) in field.map.iter().map(|(k, v)| (k.clone(), v.clone())) {
        if connected_locs.iter().any(|locs| locs.contains(&loc)) {
            // already accounted for
            continue;
        }

        let mut stack = vec![loc];
        let mut found: HashSet<Location> = once(loc).collect();
        while let Some(l) = stack.pop() {
            for l_test in [l.up(), l.down(), l.left(), l.right()] {
                if field.map.get(&l_test) == Some(&c) && !found.contains(&l_test) {
                    stack.push(l_test);
                    found.insert(l_test);
                }
            }
        }
        connected_locs.push(found);
    }

    println!("Number of regions: {}", connected_locs.len());

    let loc_by_id: HashMap<usize, HashSet<Location>> =
        connected_locs.into_iter().enumerate().collect();

    let ids: BTreeMap<Location, usize> = loc_by_id
        .iter()
        .flat_map(move |(&id, locs)| locs.iter().cloned().map(move |loc| (loc, id)))
        .collect();

    /*
    for row in 0..=field.max.row {
        for col in 0..=field.max.col {
            print!("{:0>2}|", ids.get(&Location::new(row, col)).unwrap());
        }
        println!();
    }
     */

    let areas_by_id: HashMap<usize, usize> = loc_by_id
        .iter()
        .map(|(&id, locs)| (id, locs.len()))
        .collect();
    let areas_by_id = ids.values().cloned().counts();
    //dbg!(&areas_by_id);

    let left_edges = apply_kernel3_par(&field.map, left_edge_kernel);
    let right_edges = apply_kernel3_par(&field.map, right_edge_kernel);
    let lower_edges = apply_kernel3_par(&field.map, lower_edge_kernel);
    let upper_edges = apply_kernel3_par(&field.map, upper_edge_kernel);

    let mut edge_count_by_id: HashMap<usize, usize> = HashMap::new();
    for loc in field.map.keys() {
        let id = ids.get(loc).unwrap().clone();
        let edge_count = edge_count_by_id.entry(id).or_default();
        if left_edges.get(loc).is_some() {
            *edge_count += 1;
        }
        if right_edges.get(loc).is_some() {
            *edge_count += 1;
        }
        if lower_edges.get(loc).is_some() {
            *edge_count += 1;
        }
        if upper_edges.get(loc).is_some() {
            *edge_count += 1;
        }
    }
    //dbg!(&edge_count_by_id);

    let res: u64 = areas_by_id
        .iter()
        .map(|(&id, &area)| area as u64 * *edge_count_by_id.get(&id).unwrap() as u64)
        .sum();
    println!("{}", res);

    Ok(())
}
