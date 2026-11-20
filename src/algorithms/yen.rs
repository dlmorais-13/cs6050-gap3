use std::collections::HashSet;

use petgraph::{Direction, visit::EdgeRef};

use crate::algorithms::common::{
    Graph, build_shortest_path_tree, get_initialized_control_variables, relax_edge,
};

/// Yen's algorithm.
/// 
/// Implements both early termination and predictive order to reduce the number
/// of relaxation tentatives in the whole algorithm.
pub fn yen(graph: &Graph) -> Graph {
    // Initialize control variables.
    let (mut dist_node, mut prev_node) = get_initialized_control_variables(&graph);

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
        for node in g_plus.node_indices() {
            if changed.contains(&node) || changed_this_run.contains(&node) {
                for edge in g_plus
                    .edges_directed(node, Direction::Outgoing)
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
        for node in g_minus.node_indices().rev() {
            if changed.contains(&node) || changed_this_run.contains(&node) {
                for edge in g_minus
                    .edges_directed(node, Direction::Outgoing)
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
