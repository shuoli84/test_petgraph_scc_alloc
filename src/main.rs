use alloc_counter::{count_alloc, AllocCounterSystem};
use petgraph::{prelude::NodeIndex, Graph};

#[global_allocator]
static A: AllocCounterSystem = AllocCounterSystem;

type TestGraph = Graph<usize, u8>;

fn test_graph(n: usize) -> TestGraph {
    use petgraph::prelude::*;

    let mut gr = Graph::new();
    let mut indexes = vec![];

    // add test nodes
    for i in 0..n {
        indexes.push(gr.add_node(i));
    }

    // create a snake shape graph
    for w in indexes.windows(2) {
        let from = &w[0];
        let to = &w[1];
        gr.add_edge(*from, *to, 1);
    }

    gr
}

fn tj_scc(gr: &TestGraph) -> Vec<NodeIndex> {
    use petgraph::algo::tarjan_scc;

    let sccs = tarjan_scc(gr);
    let nodes = sccs.into_iter().flatten().rev().collect::<Vec<_>>();
    nodes
}

fn scc_iter(graph: &TestGraph) -> Vec<NodeIndex> {
    let mut tarjan_scc = petgraph::algo::TarjanScc::new();

    let mut nodes = Vec::with_capacity(graph.node_count());
    tarjan_scc.run(graph, |scc| {
        nodes.extend_from_slice(scc);
    });

    nodes.reverse();

    nodes
}

fn empty_system() {}

fn test_bevy_main(graph_size: usize) {
    use bevy_ecs_main::prelude::*;

    let mut world = World::new();
    let mut schedule = Schedule::new();

    for _ in 0..graph_size {
        schedule.add_system(empty_system);
    }

    schedule.run(&mut world);
}

fn test_bevy_optimized(graph_size: usize) {
    use bevy_ecs_shuo::prelude::*;

    let mut world = World::new();
    let mut schedule = Schedule::new();

    for _ in 0..graph_size {
        schedule.add_system(empty_system);
    }

    schedule.run(&mut world);
}

fn main() {
    for i in 0..10 {
        let n = i * 100;
        println!("testing node count: {n}");
        let test_graph = test_graph(n);

        let (tj_scc_counts, tj_scc_result) = count_alloc(|| tj_scc(&test_graph));
        let (iter_scc_counts, iter_scc_result) = count_alloc(|| scc_iter(&test_graph));

        assert_eq!(tj_scc_result, iter_scc_result);
        println!("  tj_scc alloc count: {tj_scc_counts:?}");
        println!("  iter_scc_counts alloc count: {iter_scc_counts:?}");
    }

    for i in 0..10 {
        let n = i * 100;
        println!("testing bevy graph count: {n}");

        let (main_alloc_counts, _) = count_alloc(|| test_bevy_main(n));
        let (optimized_alloc_counts, _) = count_alloc(|| test_bevy_optimized(n));

        println!("  bevy_main alloc count: {main_alloc_counts:?}");
        println!("  bevy_optimized count: {optimized_alloc_counts:?}");
    }
}
