use petgraph::graph::{Graph, NodeIndex};
use petgraph::algo::{dijkstra};
use petgraph::prelude::*;

use std::collections::HashMap;
use std::fs;

fn get_topology(data: &String) -> (HashMap<(i32, i32), char>, usize, NodeIndex, NodeIndex, Vec<NodeIndex>) {
    let x: usize = data.lines().next().expect("One line").chars().count();
    let mut topology: HashMap<(i32, i32), char> = HashMap::new();
    let mut y_i: usize = 0;
    let mut start: NodeIndex = NodeIndex::new(0);
    let mut end: NodeIndex = NodeIndex::new(0);
    let mut all_a: Vec<NodeIndex> = vec![];
    for line in data.lines(){
        for (x_i, c) in line.chars().enumerate() {
            let mut ch = c;
            if c == 'S' {
                ch = 'a';
                start = NodeIndex::new(x_i + y_i*x);
            }

            if c == 'E' {
                ch = 'z';
                end = NodeIndex::new(x_i + y_i*x);
            }

            if ch == 'a' {
                all_a.push(NodeIndex::new(x_i + y_i*x));
            }

            topology.insert((x_i as i32, y_i as i32), ch);
        }
        y_i += 1;
    }
    return (topology, x, start, end, all_a);
}

fn get_graph(topology: HashMap<(i32, i32), char>, x: usize) -> Graph::<i32, i32> {
    let x = x as i32;
    let mut g = Graph::<i32, i32, Directed>::new();
    let mut vertices : Vec<NodeIndex> = vec![];

    for node in 0..topology.len(){
        vertices.push(g.add_node(node as i32));
    }

    for (coord, c) in &topology {
        let curr_char = *c;
        // Get the node number by essentially flattening the array. (0,0) -> 0 (x,0) -> x-1 (0,1) -> 0 + x
        let curr_node = coord.0 + coord.1*x;
    
        let keys = [(coord.0 - 1, coord.1), (coord.0 + 1, coord.1), (coord.0, coord.1-1), (coord.0, coord.1+1)];
        for key in keys {
            let key_node = key.0 + key.1*x;
            if topology.contains_key(&key)  {
                let neighbor: char = *topology.get(&key).unwrap();
                if neighbor <= curr_char || neighbor as u32 == curr_char as u32 + 1 {

                    // Redirect the graph so that key_node -> curr_node means that curr_node could move to the inspected node
                    // This allows one plm to be built later that shows all distances from the endpoint to 
                    g.add_edge(vertices[key_node as usize], vertices[curr_node as usize], 0);
                }
            }
        }
    }
    return g
}

fn main() {
    let data = fs::read_to_string("./src/input.txt").expect("Error reading file");
    let (topology, x, s, e, all_a) = get_topology(&data);
    let g = get_graph(topology, x);
    
    // Generate all valid paths to the endpoint by running dijkstra from enpoint -> all other nodes
    let plm = dijkstra(&g,e,None, |_| 1);

    // Path from start -> end is at plm[s]
    println!("Prob 1: {}", plm[&s]);

    // Set min path to answer from p1
    let mut prob2 = plm[&s];

    // Loop over all starting points, find their pre-computed path lengths from p1, and update min if necesarry
    for start in all_a {
        if plm.contains_key(&start){
            let pl = plm[&start];
            if pl < prob2 {
                prob2 = pl;
            }
        }
    }
    println!("Prob 2: {}", prob2);
}
