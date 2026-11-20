use rand::seq::SliceRandom;
use std::collections::HashSet;

use petgraph::{Direction, visit::EdgeRef};

use crate::algorithms::common::{
    Graph, build_shortest_path_tree, get_initialized_control_variables, relax_edge,
};

/// Yen's algorithm with random base order.
/// 
/// Modification of yen's algorithm to change the order in which the nodes are traversed.
/// In this version, the nodes are traversed in a random predefined order instead of sequentially.
pub fn random_yen(graph: &Graph) -> Graph {
    // Initialize control variables.
    let (mut dist_node, mut prev_node) = get_initialized_control_variables(&graph);

    // Create the shuffled order of vertices.
    let mut shuffled_order: Vec<_> = graph.node_indices().clone().collect();
    let mut rng = rand::rng();
    if !shuffled_order.is_empty() {
        shuffled_order[1..].shuffle(&mut rng);
    }

    let mut g_plus = graph.clone();
    g_plus.retain_edges(|g, edge| {
        let (u, v) = g.edge_endpoints(edge).unwrap();
        u < v
    });

    let mut g_minus = graph.clone();
    g_minus.retain_edges(|g, edge| {
        let (u, v) = g.edge_endpoints(edge).unwrap();
        u > v
    });

    let mut changed = HashSet::new();

    // Initialize changed set with start node.
    changed.insert(graph.node_indices().next().unwrap());

    while !changed.is_empty() {
        let mut changed_this_run = HashSet::new();

        // Iterate over nodes in G+ in numerical order.
        for node in &shuffled_order {
            if changed.contains(&node) || changed_this_run.contains(node) {
                for edge in g_plus
                    .edges_directed(*node, Direction::Outgoing)
                    .map(|e| e.id())
                {
                    if relax_edge(&g_plus, &edge, &mut dist_node, &mut prev_node) {
                        let (_, v) = g_plus.edge_endpoints(edge).unwrap();
                        changed_this_run.insert(v);
                    }
                }
            }
        }

        // Iterate over nodes in G- in reverse numerical order.
        for node in shuffled_order.iter().rev() {
            if changed.contains(&node) || changed_this_run.contains(node) {
                for edge in g_minus
                    .edges_directed(*node, Direction::Outgoing)
                    .map(|e| e.id())
                {
                    if relax_edge(&g_minus, &edge, &mut dist_node, &mut prev_node) {
                        let (_, v) = g_minus.edge_endpoints(edge).unwrap();
                        changed_this_run.insert(v);
                    }
                }
            }
        }

        changed = changed_this_run.clone();
    }

    // Remove the edges that are not part of the shortest path tree.
    build_shortest_path_tree(&graph, &prev_node)
}
