use anyhow;
use aoc23;
use itertools::*;
use petgraph::algo::all_simple_paths;
use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::dot::Config as DotConfig;
use petgraph::dot::Dot;
use petgraph::prelude::*;
use petgraph::algo::kosaraju_scc;

fn parse<'a>(lines: impl IntoIterator<Item = &'a str>) -> UnGraphMap<&'a str, ()> {
    let lines = lines.into_iter();
    let node_est = lines.size_hint().0;
    let mut graph = UnGraphMap::with_capacity(node_est, node_est * 2);
    for line in lines {
        let (node0, nodes) = line.trim().split_once(':').unwrap();
        let nodes = nodes.trim().split_whitespace();
        for node in nodes {
            graph.add_edge(node0, node, ());
        }
    }
    graph
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines().collect_vec();
    let lines = lines.iter().map(|s| s.as_str()).collect_vec();
    let mut graph = parse(lines);
    println!(
        "Input graph ({} nodes, {} edges):",
        graph.node_count(),
        graph.edge_count()
    );
    // println!(
    //     "\n\n{:?}\n\n",
    //     Dot::with_config(&graph, &[DotConfig::EdgeNoLabel])
    // );

    // s-t chosen to be in the two sides of the graph partitioning of the puzzle input
    let s = "zjv";
    assert!(graph.edges(s).count() >= 4);
    let t = "xvz";
    assert!(graph.edges(t).count() >= 4);

    // find 3 paths from s to t and remove the paths from the graph
    for i in 1..=3 {
        println!("{}: searching path s-t", i);
        // work on spanning tree to find path quicker
        let path = {
            let spanning_tree = UnGraphMap::from_elements(min_spanning_tree(&graph));
            let mut paths = all_simple_paths::<Vec<_>, _>(&spanning_tree, s, t, 0, None);
            paths.next().unwrap()
        }; // drop(paths)
        println!("{}: path s-t len: {}", i, path.len());
        for (a, b) in path.into_iter().tuple_windows() {
            graph.remove_edge(a, b).unwrap();
        }
        println!("{}: Removed path s-t", i);
    }

    // now the graph should have 2 strongly connected components, one inlc s one inlc t

    assert_eq!(kosaraju_scc(&graph).len(), 2);

    let mut scc_size_s = 0;
    let mut dfs_s = Dfs::new(&graph, s);
    while let Some(_) = dfs_s.next(&graph) {
        scc_size_s += 1;
    }

    let scc_size_t = graph.node_count() - scc_size_s;

    let sum: usize = scc_size_s * scc_size_t;
    println!("{}", sum);
    Ok(())
}
