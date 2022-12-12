use petgraph::graph::{Graph, NodeIndex};
use petgraph::algo::{astar, dijkstra};
use petgraph::prelude::*;
use petgraph::data::FromElements;
use petgraph::dot::{Dot, Config};

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
    let mut edges: Vec<(i32,i32)> = vec![];

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
                    g.add_edge(vertices[curr_node as usize], vertices[key_node as usize], 0);
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
    
    let plm = dijkstra(&g,s,Some(e), |_| 1);
    println!("Prob 1: {}", plm[&e]);

    // Loop over each starting point
    let mut prob2 = plm[&e];
    for start in all_a {
        let path_mat = dijkstra(&g,start,Some(e), |_| 1);
        if path_mat.contains_key(&e){
            let pl = path_mat[&e];
            if pl < prob2 {
                prob2 = pl;
            }
        }
    }
    println!("Prob 2: {}", prob2);
    // astar(
    //     &g,
    //     s,               // start
    //     |n| n == e,      // is_goal
    //     |e| 1, // edge_cost
    //     |_| 0,           // estimate_cost
    // );

    // match path {
    //     Some((cost, path)) => {
    //         println!("Path Length Prob 1 {}", cost);
    //     }
    //     None => println!("There was no path"),
    // }
    
    
    //println!("{:?}", topology)
}
