use std::collections::HashSet;

use petgraph::{Direction, visit::EdgeRef};

use crate::algorithms::common::{Graph, build_shortest_path_tree, get_initialized_control_variables, relax_edge};

/// Bellman-Ford with early termination.
/// 
/// Relaxes only the edges that are modified in the last iteration, starting from the starting vertex in the graph.
pub fn bellman_ford_early_termination(graph: &Graph) -> Graph {
    // Initialize control variables.
    let (mut dist_node, mut prev_node) = get_initialized_control_variables(&graph);

    let mut changed = HashSet::new();

    // Initialize changed set with start node.
    changed.insert(graph.node_indices().next().unwrap());

    while !changed.is_empty() {
        let mut changed_this_run = HashSet::new();
        for node in &changed {
            for edge in graph
                .edges_directed(*node, Direction::Outgoing)
                .map(|e| e.id())
            {
                if relax_edge(&graph, &edge, &mut dist_node, &mut prev_node) {
                    let (_, v) = graph.edge_endpoints(edge).unwrap();
                    changed_this_run.insert(v);
                }
            }
        }
        changed = changed_this_run.clone();
    }

    // Remove the edges that are not part of the shortest path tree.
    build_shortest_path_tree(&graph, &prev_node)
}
