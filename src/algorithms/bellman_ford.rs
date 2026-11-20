use crate::algorithms::common::{
    Graph, build_shortest_path_tree, get_initialized_control_variables, relax_edge,
};

/// Basic Bellman-Ford implementation
/// 
/// Tries to relax all edges once for each vertex in the graph.
pub fn bellman_ford(graph: &Graph) -> Graph {
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
