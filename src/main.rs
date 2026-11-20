// use ascii_petgraph::RenderedGraph;
use ascii_dag::{BoxedNode, Direction, Graph as DagGraph, LayoutConfig, RenderOptions};
use gap_3::{Graph, bellman_ford, paper_input};
use petgraph::visit::EdgeRef;

fn main() {
    println!("::::: Graph :::::");
    let graph = paper_input::input();
    print_graph(&graph);

    println!("::::: Shortest Path Tree :::::");
    let tree = bellman_ford(&graph);
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
