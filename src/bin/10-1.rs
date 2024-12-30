use anyhow;
use aoc24;
use itertools::Itertools;
use petgraph::algo::DfsSpace;
use petgraph::dot;
use petgraph::prelude::*;

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
    let mut space = DfsSpace::new(&graph);
    for head in trailheads {
        for &end in &trailends {
            if petgraph::algo::has_path_connecting(&graph, head, end, Some(&mut space)) {
                res += 1;
            }
        }
    }

    println!("{}", res);

    Ok(())
}
