use std::collections::HashMap;

use anyhow;
use aoc23::{self, Direction as LDirection, Location};
use itertools::*;
use petgraph::algo::all_simple_paths;
use petgraph::data::Build;
use petgraph::dot::{Config as DotConfig, Dot};
use petgraph::prelude::*;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Entry {
    Path,
    Forest,
    Slope(LDirection),
}

fn parse(lines: impl Iterator<Item = String>) -> HashMap<Location, Entry> {
    lines
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, c)| {
                    (
                        Location::new_usize(row, col),
                        match c {
                            '.' => Entry::Path,
                            '#' => Entry::Forest,
                            '<' => Entry::Slope(LDirection::Left),
                            '>' => Entry::Slope(LDirection::Right),
                            '^' => Entry::Slope(LDirection::Up),
                            'v' => Entry::Slope(LDirection::Down),
                            _ => panic!("invalid entry {}", c),
                        },
                    )
                })
                .collect_vec()
        })
        .collect()
}

struct HikeGraph {
    graph: DiGraph<Location, u32>,
    nodes: HashMap<Location, NodeIndex>,
    entry_node: NodeIndex,
    exit_node: NodeIndex,
}

fn build_graph(pattern: HashMap<Location, Entry>) -> HikeGraph {
    use LDirection::*;
    let max_row = pattern.keys().map(|loc| loc.row).max().unwrap();
    let max_col = pattern.keys().map(|loc| loc.col).max().unwrap();
    let path_count = pattern.values().filter(|&e| e != &Entry::Forest).count();

    let mut graph = DiGraph::with_capacity(path_count, path_count * 2);
    let mut nodes = HashMap::with_capacity(path_count);

    let mut entry_node = None;
    let mut exit_node = None;

    // find entry node and exit node
    for col in 0..=max_col {
        // entry node
        let loc = Location::new(0, col);
        if pattern.get(&loc) == Some(&Entry::Path) {
            let id = graph.add_node(loc);
            nodes.insert(loc, id);
            entry_node = Some(id);
        }
        // exit node
        let loc = Location::new(max_row, col);
        if pattern.get(&loc) == Some(&Entry::Path) {
            let id = graph.add_node(loc);
            nodes.insert(loc, id);
            exit_node = Some(id);
        }
    }

    // build graph

    for (&loc, entry) in pattern.iter() {
        let neighbors_dir = match entry {
            Entry::Forest => continue,
            Entry::Path => vec![Up, Down, Left, Right],
            Entry::Slope(dir) => vec![Up, Down, Left, Right],
        };
        let neighbors: Vec<Location> = neighbors_dir
            .iter()
            .filter_map(|&dir| {
                let n_loc = loc.apply(dir);
                match pattern.get(&n_loc) {
                    None => None,
                    Some(Entry::Forest) => None,
                    Some(Entry::Path) => Some(n_loc),
                    Some(Entry::Slope(_)) => Some(n_loc),
                }
            })
            .collect_vec();

        let id = *nodes.entry(loc).or_insert_with(|| graph.add_node(loc));
        for n_loc in neighbors {
            let n_id = *nodes.entry(n_loc).or_insert_with(|| graph.add_node(n_loc));
            graph.add_edge(id, n_id, 1);
        }
    }

    HikeGraph {
        graph,
        nodes,
        entry_node: entry_node.unwrap(),
        exit_node: exit_node.unwrap(),
    }
}

fn condense_graph(graph: HikeGraph) -> HikeGraph {
    println!("Warning: condense_graph not implemented");
    graph
}

fn find_longest_hike(graph: HikeGraph) -> u32 {
    let mut longest = 0;
    all_simple_paths::<Vec<_>, _>(&graph.graph, graph.entry_node, graph.exit_node, 0, None)
        // .par_bridge()
        .map(|path| {
            let mut len = 0;
            for (&a, &b) in path.iter().tuple_windows() {
                let edge = graph.graph.find_edge(a, b).unwrap();
                let edge_len = graph.graph[edge];
                len += edge_len;
            }
            len
        })
        .map(|len| {
            longest = longest.max(len);
            longest
        })
        .enumerate()
        .inspect(|(i, len)| if i % 1_000 == 0 { println!("{} -> {}", i, len); })
        .map(|(_, len)| len)
        .max()
        .unwrap()
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();
    let pattern = parse(lines);
    println!("Parsed pattern ({} entries)", pattern.len());
    let graph = build_graph(pattern);
    println!(
        "Built pattern graph ({} nodes, {} edges)",
        graph.graph.node_count(),
        graph.graph.edge_count()
    );
    let graph = condense_graph(graph);
    println!(
        "Built condensed graph ({} nodes, {} edges)",
        graph.graph.node_count(),
        graph.graph.edge_count()
    );
    // println!("Condensed graph: \n\n{:?}\n\n", Dot::new(&graph.graph));
    let sum: u32 = find_longest_hike(graph);
    println!("{}", sum);
    Ok(())
}
