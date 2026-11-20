use std::env;

use ascii_dag::{BoxedNode, Direction, Graph as DagGraph, LayoutConfig, RenderOptions};
use gap_3::{algorithms::{bellman_ford::bellman_ford, common::Graph, early_termination::bellman_ford_early_termination, random_yen::random_yen, yen::yen}, inputs::paper_input};
use petgraph::visit::EdgeRef;

fn main() {
    println!("::::: Graph :::::");
    let graph = paper_input::input();
    print_graph(&graph);

    let args: Vec<String> = env::args().collect();
    let algo = args[1].as_str();

    let tree = match algo {
        "bellman-ford" => bellman_ford(&graph),
        "early-termination" => bellman_ford_early_termination(&graph),
        "yen" => yen(&graph),
        "random-yen" => random_yen(&graph),
        _ => panic!("Unknown implementation: {}.", algo)
    };

    println!("::::: Shortest Path Tree :::::");
    print_graph(&tree);
}

/// Print graph to terminal using graph-easy.
fn print_graph(graph: &Graph) {
    let edge_labels: Vec<String> = graph
        .edge_references()
        .map(|edge| edge.weight().to_string())
        .collect();

    let mut dag = DagGraph::new();
    for idx in graph.node_indices() {
        dag.add_node(idx.index(), BoxedNode(&graph[idx]));
    }
    for (edge, label) in graph.edge_references().zip(&edge_labels) {
        dag.add_edge(
            edge.source().index(),
            edge.target().index(),
            Some(label.as_str()),
        );
    }

    let mut config = LayoutConfig::quality();
    config.direction = Direction::LeftRight;
    config.node_spacing = 6;
    config.level_spacing = 10;
    let opts = RenderOptions::default();
    let ir = dag.compute_layout_with_config(&config);
    println!("{}", ir.render_string(&opts));
}
