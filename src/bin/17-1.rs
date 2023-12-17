use std::collections::HashMap;

use anyhow;
use aoc23;
use aoc23::{Direction as Dir, Location};
use itertools::*;
use petgraph::{algo::dijkstra, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// d_-2 d_-1 d_0
struct LastDirections([Dir; 3]);

impl LastDirections {
    fn new(d_2: Dir, d_1: Dir, d_0: Dir) -> Self {
        LastDirections([d_2, d_1, d_0])
    }
    fn push(&self, dir: Dir) -> Self {
        LastDirections::new(self.0[1], self.0[2], dir)
    }
    fn all_equal_dir(&self) -> Option<Dir> {
        self.0.iter().all_equal_value().ok().copied()
    }
    fn latest(&self) -> Dir {
        self.0[2]
    }
}

type Heat = u32;
type Pattern = HashMap<Location, Heat>;

fn parse_pattern(lines: impl Iterator<Item = String>) -> Pattern {
    lines
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, c)| {
                    (
                        Location::new(row as isize, col as isize),
                        c.to_digit(10).unwrap(),
                    )
                })
                .collect_vec()
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    location: Location,
    last_directions: LastDirections,
    heat: Heat,
}

fn all_nodes_to<'a>(pattern: &'a Pattern, location: Location) -> impl Iterator<Item = Node> + 'a {
    use Dir::*;
    [Up, Down, Left, Right]
        .into_iter()
        .cartesian_product([Up, Down, Left, Right])
        .cartesian_product([Up, Down, Left, Right])
        .filter_map(move |((d_2, d_1), d_0)| {
            pattern.get(&location).copied().map(|heat| Node {
                heat,
                location,
                last_directions: LastDirections::new(d_2, d_1, d_0),
            })
        })
}

fn all_nodes<'a>(pattern: &'a Pattern) -> impl Iterator<Item = Node> + 'a {
    pattern
        .keys()
        .flat_map(|location| all_nodes_to(pattern, *location))
}

fn initial_nodes<'a>(pattern: &'a Pattern) -> impl Iterator<Item = Node> + 'a {
    all_nodes_to(pattern, Location::new(0, 0))
}

fn final_nodes<'a>(pattern: &'a Pattern) -> impl Iterator<Item = Node> + 'a {
    let max_row = pattern.keys().map(|loc| loc.row).max().unwrap();
    let max_col = pattern.keys().map(|loc| loc.col).max().unwrap();
    all_nodes_to(pattern, Location::new(max_row, max_col))
}

fn adjacent_nodes<'a>(pattern: &'a Pattern, node: &'a Node) -> impl Iterator<Item = Node> + 'a {
    use Dir::*;
    // generate all adjacent nodes that are not a 4th step in the same direction
    // and have a location in the pattern
    [Up, Down, Left, Right]
        .into_iter()
        // make sure not to go backwards
        .filter(|dir| *dir != node.last_directions.latest().rev())
        .filter_map(|dir| {
            let location = node.location.apply(dir);
            // make sure the new node is in the pattern
            pattern.get(&location).copied().map(|heat| Node {
                location,
                heat,
                last_directions: node.last_directions.push(dir),
            })
        })
        .filter(|new_node| {
            // make sure no 4th step in the same direction is taken
            if let Some(dir) = node.last_directions.all_equal_dir() {
                new_node.location != node.location.apply(dir)
            } else {
                true
            }
        })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GraphNode {
    Node(Node),
    Start,
    End,
}

fn graph_from_pattern(pattern: &Pattern) -> GraphResult {
    // size estimates
    let node_cap = 64 * pattern.len() + 2;
    let edge_cap = 8 * node_cap + 16;

    // keep map of node -> idx to find final nodes with edge to end node
    let mut idxs = HashMap::<Node, NodeIndex>::with_capacity(node_cap);
    let mut graph = Graph::with_capacity(node_cap, edge_cap);

    // generate start and end nodes
    let start_idx = graph.add_node(GraphNode::Start);
    let end_idx = graph.add_node(GraphNode::End);

    // add all nodes for pattern
    for node in all_nodes(pattern) {
        let idx = graph.add_node(GraphNode::Node(node));
        idxs.insert(node, idx);
    }

    // add heat edges to adjacent nodes foreach node
    for (node_0, idx_0) in idxs.iter() {
        for (node_1, idx_1) in adjacent_nodes(pattern, node_0).map(|n| (n, idxs.get(&n).unwrap())) {
            graph.add_edge(*idx_0, *idx_1, node_1.heat);
        }
    }

    // add Start node with 0 edges to initial nodes
    for (_node_i, idx_i) in initial_nodes(pattern).map(|n| (n, idxs.get(&n).unwrap())) {
        graph.add_edge(start_idx, *idx_i, 0);
    }

    // add End node with 0 edges from final nodes
    for (_node_f, idx_f) in final_nodes(pattern).map(|n| (n, idxs.get(&n).unwrap())) {
        graph.add_edge(*idx_f, end_idx, 0);
    }

    GraphResult {
        graph,
        start_idx,
        end_idx,
    }
}

#[derive(Clone)]
struct GraphResult {
    graph: DiGraph<GraphNode, Heat>,
    start_idx: NodeIndex,
    end_idx: NodeIndex,
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();
    let pattern = parse_pattern(lines);

    println!("parsed input");
    println!("input len: {}", pattern.len());

    let graph_res = graph_from_pattern(&pattern);

    println!("generated super graph");
    println!("nodes: {}", graph_res.graph.node_count());
    println!("edges: {}", graph_res.graph.edge_count());

    let d_map = dijkstra(
        &graph_res.graph,
        graph_res.start_idx,
        Some(graph_res.end_idx),
        |e| *e.weight(),
    );

    let sum: u32 = *d_map.get(&graph_res.end_idx).unwrap();
    println!("{}", sum);
    Ok(())
}
