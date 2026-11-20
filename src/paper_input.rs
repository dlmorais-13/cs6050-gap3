use petgraph::graph::DiGraph;

use crate::Graph;

/// Builds the graph used in the paper.
pub fn input() -> Graph {
    let mut g = DiGraph::<String, i32>::new();

    let mut nodes = vec![];
    for i in 0..10 {
        nodes.push(g.add_node(i.to_string()));
    }

    g.add_edge(nodes[0], nodes[2], 3);
    g.add_edge(nodes[0], nodes[5], 5);
    g.add_edge(nodes[0], nodes[9], 9);

    g.add_edge(nodes[1], nodes[4], 9);

    g.add_edge(nodes[2], nodes[7], 1);

    g.add_edge(nodes[3], nodes[4], 2);
    g.add_edge(nodes[3], nodes[8], 1);

    g.add_edge(nodes[4], nodes[5], 0);
    g.add_edge(nodes[4], nodes[8], -8);

    g.add_edge(nodes[5], nodes[1], 2);

    g.add_edge(nodes[6], nodes[1], 0);

    g.add_edge(nodes[7], nodes[3], 0);
    g.add_edge(nodes[7], nodes[9], 2);

    g.add_edge(nodes[8], nodes[2], 8);
    g.add_edge(nodes[8], nodes[6], 2);

    g.add_edge(nodes[9], nodes[4], 3);

    g
}
