use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_graph_from_file(filename: &str) -> HashMap<String, Vec<String>> {
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            let node = parts[0].trim().to_string();
            let edges: Vec<String> = parts[1].split(',').map(|s| s.trim().to_string()).collect();
            graph.insert(node, edges);
        }
    }

    graph
}

fn dfs<'a>(
    graph: &'a HashMap<String, Vec<String>>,
    start: &'a str,
    visited: &mut HashSet<&'a str>,
    spanning_tree: &mut HashMap<&'a str, Vec<&'a str>>,
) {
    if visited.contains(start) {
        return;
    }

    visited.insert(start);
    if let Some(neighbors) = graph.get(start) {
        for neighbor in neighbors {
            if !visited.contains(neighbor.as_str()) {
                spanning_tree
                    .entry(start)
                    .or_insert(Vec::new())
                    .push(neighbor);
                dfs(graph, neighbor, visited, spanning_tree);
            }
        }
    }
}

fn main() {
    // Read the graph from the file
    let graph = read_graph_from_file("graph.txt");

    // Prepare for DFS
    let start_node = "A"; // You can change the start node as needed
    let mut visited: HashSet<&str> = HashSet::new();
    let mut spanning_tree: HashMap<&str, Vec<&str>> = HashMap::new();

    // Run DFS
    dfs(&graph, start_node, &mut visited, &mut spanning_tree);

    // Print the DFS spanning tree
    println!("DFS Spanning Tree:");
    for (node, edges) in spanning_tree {
        println!("{}: {:?}", node, edges);
    }
}
