use alloc_counter::{count_alloc, AllocCounterSystem};
use petgraph::{prelude::NodeIndex, Graph};

#[global_allocator]
static A: AllocCounterSystem = AllocCounterSystem;

type TestGraph = Graph<&'static str, f64>;

fn test_graph() -> TestGraph {
    use petgraph::prelude::*;

    let mut gr = Graph::new();
    let h = gr.add_node("H");
    let i = gr.add_node("I");
    let j = gr.add_node("J");
    let k = gr.add_node("K");
    // Z is disconnected.
    let _ = gr.add_node("Z");
    gr.add_edge(h, i, 1.);
    gr.add_edge(h, j, 3.);
    gr.add_edge(i, j, 1.);
    gr.add_edge(i, k, 2.);

    gr
}

fn tj_scc(gr: &TestGraph) -> Vec<NodeIndex> {
    use petgraph::algo::tarjan_scc;

    let sccs = tarjan_scc(gr);
    let nodes = sccs.into_iter().flatten().rev().collect::<Vec<_>>();
    assert_eq!(nodes.len(), 5);
    nodes
}

fn scc_iter(graph: &TestGraph) -> Vec<NodeIndex> {
    let mut tarjan_scc = petgraph::algo::TarjanScc::new();

    let mut nodes = Vec::with_capacity(graph.node_count());
    tarjan_scc.run(graph, |scc| {
        nodes.extend_from_slice(scc);
    });

    nodes.reverse();

    assert_eq!(nodes.len(), 5);
    nodes
}

fn main() {
    let test_graph = test_graph();

    let (tj_scc_counts, tj_scc_result) = count_alloc(|| tj_scc(&test_graph));
    let (iter_scc_counts, iter_scc_result) = count_alloc(|| scc_iter(&test_graph));

    assert_eq!(tj_scc_result, iter_scc_result);
    println!("tj_scc alloc count: {tj_scc_counts:?}");
    println!("iter_scc_counts alloc count: {iter_scc_counts:?}");
}
