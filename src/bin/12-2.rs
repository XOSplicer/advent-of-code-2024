use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    iter::once,
};

use anyhow;
use aoc24::{self, Distance, Location};
use itertools::Itertools;
use rayon::prelude::*;

struct Kernel3Input<'a, T> {
    /// row-major kernel view
    inner: [[Option<&'a T>; 3]; 3],
}

impl<'a, T> Kernel3Input<'a, T> {
    fn center(&self) -> &'a T {
        self.inner[1][1].unwrap()
    }
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

fn upper_left_outer_corner_kernel(input: Kernel3Input<char>) -> Option<char> {
    /*
       x o x
       o c x
       x x x
    */
    if input.inner[0][1] != Some(input.center()) && input.inner[1][0] != Some(input.center()) {
        return Some(*input.center());
    }
    None
}

fn lower_left_outer_corner_kernel(input: Kernel3Input<char>) -> Option<char> {
    /*
       x x x
       o c x
       x o x
    */
    if input.inner[2][1] != Some(input.center()) && input.inner[1][0] != Some(input.center()) {
        return Some(*input.center());
    }
    None
}

fn upper_right_outer_corner_kernel(input: Kernel3Input<char>) -> Option<char> {
    /*
       x o x
       x c o
       x x x
    */
    if input.inner[0][1] != Some(input.center()) && input.inner[1][2] != Some(input.center()) {
        return Some(*input.center());
    }
    None
}

fn lower_right_outer_corner_kernel(input: Kernel3Input<char>) -> Option<char> {
    /*
       x x x
       x c o
       x o x
    */
    if input.inner[2][1] != Some(input.center()) && input.inner[1][2] != Some(input.center()) {
        return Some(*input.center());
    }
    None
}

fn upper_left_inner_corner_kernel(input: Kernel3Input<char>) -> Option<char> {
    /*
       x x x
       x c c
       x c o
    */
    if input.inner[1][2] == Some(input.center())
        && input.inner[2][1] == Some(input.center())
        && input.inner[2][2] != Some(input.center())
    {
        return Some(*input.center());
    }
    None
}

fn upper_right_inner_corner_kernel(input: Kernel3Input<char>) -> Option<char> {
    /*
       x x x
       c c x
       o c x
    */
    if input.inner[1][0] == Some(input.center())
        && input.inner[2][1] == Some(input.center())
        && input.inner[2][0] != Some(input.center())
    {
        return Some(*input.center());
    }
    None
}

fn lower_right_inner_corner_kernel(input: Kernel3Input<char>) -> Option<char> {
    /*
       o c x
       c c x
       x x x
    */
    if input.inner[0][1] == Some(input.center())
        && input.inner[1][0] == Some(input.center())
        && input.inner[0][0] != Some(input.center())
    {
        return Some(*input.center());
    }
    None
}

fn lower_left_inner_corner_kernel(input: Kernel3Input<char>) -> Option<char> {
    /*
       x c o
       x c c
       x x x
    */
    if input.inner[0][1] == Some(input.center())
        && input.inner[1][2] == Some(input.center())
        && input.inner[0][2] != Some(input.center())
    {
        return Some(*input.center());
    }
    None
}

fn main() -> anyhow::Result<()> {
    let lines = aoc24::read_input_lines();
    let field = aoc24::read_visual_map(lines);

    // use floodfill to generate ids of connected locations

    let mut loc_rest: BTreeSet<Location> = field.map.keys().cloned().collect();
    let mut connected_locs: Vec<HashSet<Location>> = Vec::new();
    while let Some(loc) = loc_rest.pop_first() {
        let c = field.map.get(&loc).unwrap();
        let mut stack = vec![loc];
        let mut found: HashSet<Location> = once(loc).collect();
        while let Some(l) = stack.pop() {
            for l_test in [l.up(), l.down(), l.left(), l.right()] {
                if field.map.get(&l_test) == Some(&c) && !found.contains(&l_test) {
                    stack.push(l_test);
                    found.insert(l_test);
                    loc_rest.remove(&l_test);
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
    let areas_by_id = ids.values().cloned().counts();
    //dbg!(&areas_by_id);

    let upper_left_outer_corners = apply_kernel3_par(&field.map, upper_left_outer_corner_kernel);
    let upper_right_outer_corners = apply_kernel3_par(&field.map, upper_right_outer_corner_kernel);
    let lower_left_outer_corners = apply_kernel3_par(&field.map, lower_left_outer_corner_kernel);
    let lower_right_outer_corners = apply_kernel3_par(&field.map, lower_right_outer_corner_kernel);
    let upper_left_inner_corners = apply_kernel3_par(&field.map, upper_left_inner_corner_kernel);
    let upper_right_inner_corners = apply_kernel3_par(&field.map, upper_right_inner_corner_kernel);
    let lower_left_inner_corners = apply_kernel3_par(&field.map, lower_left_inner_corner_kernel);
    let lower_right_inner_corners = apply_kernel3_par(&field.map, lower_right_inner_corner_kernel);

    let mut corner_count_by_id: HashMap<usize, usize> = HashMap::new();
    for loc in field.map.keys() {
        let id = ids.get(loc).unwrap().clone();
        let corner_count = corner_count_by_id.entry(id).or_default();

        if upper_left_outer_corners.get(loc).is_some() {
            *corner_count += 1;
        }
        if upper_right_outer_corners.get(loc).is_some() {
            *corner_count += 1;
        }
        if lower_left_outer_corners.get(loc).is_some() {
            *corner_count += 1;
        }
        if lower_right_outer_corners.get(loc).is_some() {
            *corner_count += 1;
        }
        if upper_left_inner_corners.get(loc).is_some() {
            *corner_count += 1;
        }
        if upper_right_inner_corners.get(loc).is_some() {
            *corner_count += 1;
        }
        if lower_left_inner_corners.get(loc).is_some() {
            *corner_count += 1;
        }
        if lower_right_inner_corners.get(loc).is_some() {
            *corner_count += 1;
        }
    }
    // dbg!(&corner_count_by_id);

    let res: u64 = areas_by_id
        .iter()
        .map(|(&id, &area)| area as u64 * *corner_count_by_id.get(&id).unwrap() as u64)
        .sum();
    println!("{}", res);

    Ok(())
}
