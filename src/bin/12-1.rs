use std::collections::BTreeMap;

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
        (Some(o), Some(c)) => (o == c).then_some(*c),
    }
}

fn right_edge_kernel(input: Kernel3Input<char>) -> Option<char> {
    match (input.inner[1][2], input.inner[1][1]) {
        (None, None) => None,
        (None, Some(c)) => Some(*c),
        (Some(_), None) => None,
        (Some(o), Some(c)) => (o == c).then_some(*c),
    }
}

fn upper_edge_kernel(input: Kernel3Input<char>) -> Option<char> {
    match (input.inner[0][1], input.inner[1][1]) {
        (None, None) => None,
        (None, Some(c)) => Some(*c),
        (Some(_), None) => None,
        (Some(o), Some(c)) => (o == c).then_some(*c),
    }
}

fn lower_edge_kernel(input: Kernel3Input<char>) -> Option<char> {
    match (input.inner[2][1], input.inner[1][1]) {
        (None, None) => None,
        (None, Some(c)) => Some(*c),
        (Some(_), None) => None,
        (Some(o), Some(c)) => (o == c).then_some(*c),
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc24::read_input_lines();

    let field = read_visual_map(lines);

    let left_edges = apply_kernel3_par(&BTreeMap::new(), left_edge_kernel);
    let right_edges = apply_kernel3_par(&BTreeMap::new(), right_edge_kernel);
    let lower_edges = apply_kernel3_par(&BTreeMap::new(), lower_edge_kernel);
    let upper_edges = apply_kernel3_par(&BTreeMap::new(), upper_edge_kernel);

    let res: u64 = 0;
    println!("{}", res);

    Ok(())
}
