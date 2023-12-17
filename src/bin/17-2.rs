use std::collections::HashMap;

use anyhow;
use aoc23;
use itertools::*;
use petgraph::{algo::dijkstra, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    row: isize,
    col: isize,
}

impl Location {
    fn new(row: isize, col: isize) -> Self {
        Location { row, col }
    }
    fn up(&self) -> Location {
        Location {
            row: self.row - 1,
            col: self.col,
        }
    }
    fn down(&self) -> Location {
        Location {
            row: self.row + 1,
            col: self.col,
        }
    }
    fn right(&self) -> Location {
        Location {
            row: self.row,
            col: self.col + 1,
        }
    }
    fn left(&self) -> Location {
        Location {
            row: self.row,
            col: self.col - 1,
        }
    }
    fn apply(&self, dir: Dir) -> Self {
        match dir {
            Dir::Up => self.up(),
            Dir::Down => self.down(),
            Dir::Right => self.right(),
            Dir::Left => self.left(),
        }
    }
    fn apply_n(&self, dir: Dir, n: usize) -> Self {
        let mut s = *self;
        for _ in 0..n {
            s = s.apply(dir);
        }
        s
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

impl Dir {
    fn rev(&self) -> Dir {
        use Dir::*;
        match self {
            Up => Down,
            Down => Up,
            Right => Left,
            Left => Right,
        }
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
    last_dir: Dir,
    heat: Heat,
}

fn all_nodes_to<'a>(pattern: &'a Pattern, location: Location) -> impl Iterator<Item = Node> + 'a {
    use Dir::*;
    [Up, Down, Left, Right]
        .into_iter()
        .filter_map(move |last_dir| {
            pattern.get(&location).copied().map(|heat| Node {
                heat,
                location,
                last_dir,
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

struct AdjacentNode {
    total_heat: Heat,
    node: Node,
}

fn total_heat_from_to(pattern: &Pattern, from: Location, to: Location) -> Heat {
    if from.col == to.col {
        let a = from.row.min(to.row);
        let b = from.row.max(to.row);
        let heat_sum = (a..=b)
            .map(|row| *pattern.get(&Location::new(row, from.col)).unwrap())
            .sum::<Heat>();
        let from_heat = pattern.get(&from).unwrap();
        return heat_sum - from_heat;
    } else if from.row == to.row {
        let a = from.col.min(to.col);
        let b = from.col.max(to.col);
        let heat_sum = (a..=b)
            .map(|col| *pattern.get(&Location::new(from.row, col)).unwrap())
            .sum::<Heat>();
        let from_heat = pattern.get(&from).unwrap();
        return heat_sum - from_heat;
    } else {
        panic!(
            "invalid total heat from to pattern: from {:?} to {:?}",
            from, to
        );
    }
}

fn adjacent_nodes<'a>(
    pattern: &'a Pattern,
    node: &'a Node,
    min_step: usize,
    max_step: usize,
) -> impl Iterator<Item = AdjacentNode> + 'a {
    use Dir::*;

    // generate all nodes that are reachable in min..=max steps,
    // that are not in the same or reverse directions
    // add up the total heat over the pattern of the nodes that would be skipped

    [Up, Down, Left, Right]
        .into_iter()
        // make sure not to go further in the same direction
        .filter(|dir| *dir != node.last_dir)
        // make sure not to go backwards
        .filter(|dir| *dir != node.last_dir.rev())
        .flat_map(move |dir| {
            (min_step..=max_step)
                .filter_map(|step| {
                    let location = node.location.apply_n(dir, step);
                    pattern.get(&location).copied().map(|heat| AdjacentNode {
                        node: Node {
                            location,
                            last_dir: dir,
                            heat,
                        },
                        total_heat: total_heat_from_to(pattern, node.location, location),
                    })
                })
                .collect_vec()
        })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GraphNode {
    Node(Node),
    Start,
    End,
}

fn graph_from_pattern(pattern: &Pattern, min_step: usize, max_step: usize) -> GraphResult {
    // size estimates
    let node_cap = 4 * pattern.len() + 2;
    let edge_cap = (max_step - min_step + 1) * node_cap * 2;

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
        for adj_node in adjacent_nodes(pattern, node_0, min_step, max_step) {
            let idx_1 = idxs.get(&adj_node.node).unwrap();
            graph.add_edge(*idx_0, *idx_1, adj_node.total_heat);
        }
    }

    // add Start node with 0 edges to initial nodes
    for node_i in initial_nodes(pattern) {
        let idx_i = idxs.get(&node_i).unwrap();
        graph.add_edge(start_idx, *idx_i, 0);
    }

    // add End node with 0 edges from final nodes
    for node_f in final_nodes(pattern) {
        let idx_f = idxs.get(&node_f).unwrap();
        graph.add_edge(*idx_f, end_idx, 0);
    }

    GraphResult {
        graph,
        start_idx,
        end_idx,
        idxs,
    }
}

#[derive(Clone)]
struct GraphResult {
    graph: DiGraph<GraphNode, Heat>,
    start_idx: NodeIndex,
    end_idx: NodeIndex,
    idxs: HashMap<Node, NodeIndex>,
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();
    let pattern = parse_pattern(lines);

    println!("parsed input");
    println!("input len: {}", pattern.len());

    // change this to (1, 3) for part 1
    let graph_res = graph_from_pattern(&pattern, 4, 10);

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
