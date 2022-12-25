use petgraph::graph::{Graph, NodeIndex};
use petgraph::algo::{dijkstra};
use petgraph::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use petgraph::dot::{Dot, Config};
use std::fmt;
use itertools::Itertools;

struct Node {
    name: String,
    flow: i32
}

impl Node {
    fn new(name: String, flow: i32) -> Node{
        Node {
            name: name,
            flow: flow
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.name, self.flow)
    }
}

#[derive(Clone)]
#[derive(Eq, Hash, PartialEq)]
struct State {
    pos: String,
    time: i32,
    opened: Vec<(i32, i32)>,
    unopened: Vec<String>,
    path: Vec<String>
}

impl State {
    fn new(start: String, t: i32, non_zero: Vec<String>, open: Vec<(i32,i32)>) -> State{
        State {
            pos: start.clone(),
            time: t,
            unopened: non_zero,
            opened: open,
            path: vec![]
        }
    }
    
    fn mv(&self, mov: String,  t: i32, non_zero: Vec<String>, open: Vec<(i32,i32)>) -> State{
        let mut new_path = self.path.clone();
        new_path.push(mov.clone());
        State {
            pos: mov,
            time: t,
            unopened: non_zero,
            opened: open,
            path: new_path
        }
    }

}




fn create_graph(tunnels: &String) -> (Graph::<Node, i32, Directed>, HashMap<String, NodeIndex>, Vec<String>){
    let re = Regex::new(r"[\D]+ ([A-Z]+)[\D]+([\d]+)[a-z ;]+([A-Z, ]*)").unwrap();
    let mut g = Graph::<Node, i32, Directed>::new();
    let mut v : HashMap<String, NodeIndex> = HashMap::new();
    let mut non_zero_v: Vec<String> = vec![];
    let mut edges: HashMap<NodeIndex, Vec<String>> = HashMap::new();

    for cap in re.captures_iter(tunnels){
        let name = &cap[1];
        let flow = &cap[2];
        let node = Node::new(name.to_string(), flow.parse::<i32>().unwrap_or(0));

        let paths: Vec<String> = cap[3].split(", ").map(|x| x.to_string()).collect();
        let v_i = g.add_node(node);
        v.insert(name.to_string(), v_i);
        edges.insert(v_i, paths);

        if flow.parse::<i32>().unwrap_or(0) != 0 {
            non_zero_v.push(name.to_string());
        } 

    }

    for (v_i, e_arr) in edges {
        for e in e_arr{
            g.add_edge(v_i, v[&e], 1);
        }
    }
    println!("{}", Dot::new(&g)); 
    return (g, v, non_zero_v);
}

fn update_state(initial: &State, to_str: String, t_spent: i32, pressure: i32) -> State{
    let mut opened = initial.opened.clone();
    let mut unopened = initial.unopened.clone();
    let t = initial.time - t_spent;
    opened.push((pressure, t));
    let index = unopened.iter().position(|x| *x == to_str).unwrap();
    unopened.remove(index);
    return initial.mv(to_str, t, unopened, opened);
}

fn iterate_states(g: &Graph::<Node, i32, Directed>, v: &HashMap<String, NodeIndex>, states: HashSet<State>) -> (HashSet<State>,HashSet<State>){
    let mut new_states: HashSet<State> = HashSet::new();
    let mut finished: HashSet<State> = HashSet::new();
    
    for state in states {
        let plm = dijkstra(g, v[&state.pos], None, |_| 1);

        if state.unopened.len() == 0 {
            finished.insert(state.clone());
        }

        for vi in &state.unopened{
            let time_spent = plm[&v[vi]]+1;
            if state.time-time_spent <= 0 {
                finished.insert(state.clone());
            }
            else{
                let final_dest = vi;
                let final_dest_idx = v[vi];
                let pressure = g.node_weight(final_dest_idx).unwrap().flow;
                let new_state = update_state(&state, final_dest.to_string(),  time_spent, pressure);
                new_states.insert(new_state);
            }
            
        }
    }

    return (new_states, finished);
}

fn get_pressure(opened: &Vec<(i32, i32)>) -> i32{
    let mut total = 0;
    for valve in opened{
        let open_t = valve.1;
        total += valve.0 * open_t;
    }

    return total;
}

fn disjoint(s1: &State, s2: &State) -> bool{
    let s1_path: HashSet<String> = HashSet::from_iter(s1.path.iter().cloned());
    let s2_path: HashSet<String> = HashSet::from_iter(s2.path.iter().cloned());
    return s1_path.is_disjoint(&s2_path);
}

fn main() {
    let tunnels = fs::read_to_string("./src/input.txt").expect("Error reading file");
    let (g, v, non_zero) = create_graph(&tunnels);
    let start: String = "AA".to_string();
    let mut states: HashSet<State> = HashSet::new();
    let mut finished_states: HashSet<State> = HashSet::new();
    let mut all_states: HashSet<State> = HashSet::new();
    

    states.insert(State::new(start.clone(), 26, non_zero.clone(), vec![]));
    let mut max_p = 0;

    while states.len() > 0{
        println!("{}", all_states.len());
        (states, finished_states) = iterate_states(&g,&v,states);
        for state in &states{
            all_states.insert(state.clone());
        }
        for state in finished_states{
            all_states.insert(state);
        }
    }

    // Find non_intersecting paths and calculate the sum of their pressure release
    for pair in all_states.into_iter().combinations(2){
        let s1 = pair.first().unwrap();
        let s2 = pair.last().unwrap();
        if disjoint(s1, s2){
            let total_p = get_pressure(&s1.opened)+get_pressure(&s2.opened);

            if total_p > max_p{
                max_p = total_p;
                println!("Pressure {:?}", max_p);
                println!("Path 1 {:?} Path 2 {:?}", s1.path, s2.path);
            }
        }
    }

    println!("Pressure {:?}", max_p);
     
    
}
