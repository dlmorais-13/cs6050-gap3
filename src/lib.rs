use infinitable::*;
use petgraph::graph::{DiGraph, EdgeIndex};

pub type Graph = DiGraph<String, i32>;

pub mod paper_input;

pub fn bellman_ford(graph: &Graph) -> DiGraph<String, i32> {
    // Initialize control variables.
    let (mut dist_node, mut prev_node) = get_initialized_control_variables(&graph);

    // Base algorithm.
    for _ in graph.node_indices() {
        for edge in graph.edge_indices() {
            relax_edge(&graph, &edge, &mut dist_node, &mut prev_node);
        }
    }

    // Remove the edges that are not part of the shortest path tree.
    build_shortest_path_tree(&graph, &prev_node)
}

/// Creates the initialized control variables for the algorithms.
fn get_initialized_control_variables(graph: &Graph) -> (Vec<Infinitable<i32>>, Vec<Option<usize>>) {
    let mut dist_node: Vec<Infinitable<i32>> = vec![];
    let mut prev_node: Vec<Option<usize>> = vec![];
    graph.node_indices().for_each(|idx| {
        dist_node.push(if idx.index() == 0 {
            Finite(0)
        } else {
            Infinity
        });
        prev_node.push(None);
    });

    (dist_node, prev_node)
}

/// Common step to relax an edge.
fn relax_edge(
    graph: &Graph,
    edge: &EdgeIndex,
    dist_node: &mut Vec<Infinitable<i32>>,
    prev_node: &mut Vec<Option<usize>>,
) {
    let (u, v) = graph.edge_endpoints(*edge).unwrap();
    let edge_weight = graph.edge_weight(*edge).unwrap();
    let du = dist_node[u.index()];
    let dv = dist_node[v.index()];

    if dv > du + Finite(*edge_weight) {
        dist_node[v.index()] = du + Finite(*edge_weight);
        prev_node[v.index()] = Some(u.index());
    }
}

/// Build the shortest path tree resulting from the algorithm's computation.
fn build_shortest_path_tree(graph: &Graph, prev_node: &Vec<Option<usize>>) -> Graph {
    let mut tree = graph.clone();
    tree.retain_edges(|g, edge| {
        let (u, v) = g.edge_endpoints(edge).unwrap();
        prev_node[v.index()] == Some(u.index())
    });
    tree
}
