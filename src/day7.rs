use std::collections::HashMap;

use petgraph::{
    visit::{Bfs, EdgeRef, Reversed},
    Graph,
};
use regex::Regex;

#[aoc_generator(day7)]
fn graph(input: &str) -> Graph<String, usize> {
    let mut graph = Graph::new();
    let mut nodes = HashMap::new();

    let root_bag_re = Regex::new("^(?P<color>[a-z]+ [a-z]+)").unwrap();
    let edge_bag_re = Regex::new("(?P<num>[0-9]) (?P<color>[a-z]+ [a-z]+) (bags|bag)").unwrap();

    for line in input.lines() {
        let caps = root_bag_re.captures(line).expect("invalid line :(");
        let color = caps.name("color").unwrap().as_str();
        let root_node = *nodes
            .entry(color)
            .or_insert_with(|| graph.add_node(color.to_string()));

        for caps in edge_bag_re.captures_iter(line) {
            let weight = caps
                .name("num")
                .unwrap()
                .as_str()
                .parse()
                .expect("Parse error");
            let color = caps.name("color").unwrap().as_str();
            let node = *nodes
                .entry(color)
                .or_insert_with(|| graph.add_node(color.to_string()));
            graph.add_edge(root_node, node, weight);
        }
    }

    graph
}

#[aoc(day7, part1)]
fn first(graph: &Graph<String, usize>) -> usize {
    let shiny_gold_bag = graph
        .node_indices()
        .find(|i| graph[*i] == "shiny gold")
        .unwrap();
    let mut count_connecting = 0;

    let graph = Reversed(graph);
    let mut bfs = Bfs::new(graph, shiny_gold_bag);
    bfs.next(&graph);
    while bfs.next(&graph).is_some() {
        count_connecting += 1;
    }

    count_connecting
}

#[aoc(day7, part2)]
fn second(graph: &Graph<String, usize>) -> usize {
    let shiny_gold_bag = graph
        .node_indices()
        .find(|i| graph[*i] == "shiny gold")
        .unwrap();
    let mut count_contained = 0;

    let mut nodes_to_visit = graph
        .edges(shiny_gold_bag)
        .map(|e| (e.target(), *e.weight()))
        .collect::<Vec<_>>();

    while !nodes_to_visit.is_empty() {
        let mut next_nodes_to_visit = vec![];

        for (node, mult) in nodes_to_visit {
            count_contained += mult;
            next_nodes_to_visit.extend(graph.edges(node).map(|e| (e.target(), mult * *e.weight())));
        }
        nodes_to_visit = next_nodes_to_visit;
    }

    count_contained
}
