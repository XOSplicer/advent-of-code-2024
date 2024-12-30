use std::marker::PhantomData;

use anyhow;
use aoc24;
use itertools::Itertools;
use petgraph::algo::DfsSpace;
use petgraph::dot;
use petgraph::prelude::*;

struct Discard<T> {
    _marker: PhantomData<T>,
}

impl<T> FromIterator<T> for Discard<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        /* noop */
        Discard {
            _marker: PhantomData,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let lines = aoc24::read_input_lines();
    let field = aoc24::read_visual_map_filter_map(lines, |c: char| match c {
        '.' => None,
        _ => Some(c.to_digit(10).unwrap()),
    });

    //   dbg!(&field);

    let mut graph = DiGraphMap::with_capacity(field.map.len(), field.map.len() * 4);

    for &loc in field.map.keys() {
        graph.add_node(loc);
    }

    for (loc, height) in field.map.iter() {
        for other_loc in [loc.up(), loc.down(), loc.left(), loc.right()] {
            if field.map.get(&other_loc).copied() == Some(height + 1) {
                //                 println!(
                //                     "Adding edge from {:?}({}) to {:?}({:?})",
                //                     loc,
                //                     height,
                //                     other_loc,
                //                     field.map.get(&other_loc)
                //                 );
                graph.add_edge(*loc, other_loc, 1);
            }
        }
    }

    println!(
        "{:?}",
        dot::Dot::with_config(&graph, &[dot::Config::EdgeNoLabel])
    );

    let trailheads = field
        .map
        .iter()
        .filter(|(_loc, height)| **height == 0)
        .map(|(loc, _height)| *loc)
        .collect_vec();
    let trailends = field
        .map
        .iter()
        .filter(|(_loc, height)| **height == 9)
        .map(|(loc, _height)| *loc)
        .collect_vec();

    let mut res: u32 = 0;
    for head in trailheads {
        for &end in &trailends {
            res += petgraph::algo::all_simple_paths::<Discard<_>, _>(&graph, head, end, 8, Some(8))
                .count() as u32;
        }
    }

    println!("{}", res);

    Ok(())
}
