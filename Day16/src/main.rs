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
struct State {
    pos: String,
    idx: NodeIndex,
    elephant: String,
    ele_idx: NodeIndex,
    time: i32,
    time_e: i32,
    opened: Vec<(i32, i32)>,
    unopened: HashSet<String>
}

impl State {
    fn new(start: String, node: NodeIndex, t: i32, non_zero: HashSet<String>, open: Vec<(i32,i32)>) -> State{
        State {
            pos: start.clone(),
            elephant: start.clone(),
            ele_idx: node,
            idx: node,
            time: t,
            time_e: t,
            unopened: non_zero,
            opened: open
        }
    }
    
    fn move_human(&self, mv: String, node: NodeIndex, t: i32, non_zero: HashSet<String>, open: Vec<(i32,i32)>) -> State{
        State {
            pos: mv,
            idx: node,
            elephant: self.elephant.clone(),
            ele_idx: self.ele_idx.clone(),
            time: t,
            time_e: self.time_e,
            unopened: non_zero,
            opened: open
        }
    }

    fn move_ele(&self, mv: String, node: NodeIndex, t: i32, non_zero: HashSet<String>, open: Vec<(i32,i32)>) -> State{
        State {
            pos: self.pos.clone(),
            idx: self.idx.clone(),
            elephant: mv,
            ele_idx: node,
            time: self.time,
            time_e: t,
            unopened: non_zero,
            opened: open
        }
    }
}




fn create_graph(tunnels: &String) -> (Graph::<Node, i32, Directed>, HashMap<String, NodeIndex>, HashSet<String>){
    let re = Regex::new(r"[\D]+ ([A-Z]+)[\D]+([\d]+)[a-z ;]+([A-Z, ]*)").unwrap();
    let mut g = Graph::<Node, i32, Directed>::new();
    let mut v : HashMap<String, NodeIndex> = HashMap::new();
    let mut non_zero_v: HashSet<String> = HashSet::new();
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
            non_zero_v.insert(name.to_string());
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

fn update_state(initial: &State, to_str: String, to_idx: NodeIndex, t_spent: i32, pressure: i32) -> State{
    let mut opened = initial.opened.clone();
    let mut unopened = initial.unopened.clone();
    let t = initial.time - t_spent;
    opened.push((pressure, t));
    unopened.remove(&to_str);
    State::new(to_str, to_idx, t, unopened, opened)
}

fn iterate_states(g: &Graph::<Node, i32, Directed>, v: &HashMap<String, NodeIndex>, states: Vec<State>) -> (Vec<State>,Vec<Vec<(i32,i32)>>){
    let mut new_states: Vec<State> = vec![];
    let mut finished: Vec<Vec<(i32,i32)>> = vec![];
    
    for state in states {
        let plm = dijkstra(g, v[&state.pos], None, |_| 1);

        if state.unopened.len() == 0 {
            finished.push(state.opened.clone());
        }

        for vi in &state.unopened{
            let time_spent = plm[&v[vi]]+1;
            if state.time-time_spent <= 0 {
                finished.push(state.opened.clone());
            }
            else{
                let final_dest = vi;
                let final_dest_idx = v[vi];
                let pressure = g.node_weight(final_dest_idx).unwrap().flow;
                let new_state = update_state(&state, final_dest.to_string(), final_dest_idx, time_spent, pressure);
                new_states.push(new_state);
            }
            
        }
    }

    return (new_states, finished);
}

fn try_move(g: &Graph::<Node, i32, Directed>, v: &HashMap<String, NodeIndex>, state: &State, vi: String, plm: &HashMap<NodeIndex, i32>, ele: bool) -> (State, bool) {
    let time_spent = plm[&v[&vi]]+1;
    let time = if ele {state.time_e - time_spent} else {state.time - time_spent};
    if time-time_spent < 0 {
        let mut finished = state.clone();
        if ele {finished.time_e = 0;} else {finished.time = 0}
        return (finished, true);
    }
    else{
        let final_dest_idx = v[&vi];
        let pressure = g.node_weight(final_dest_idx).unwrap().flow;
        let mut opened = state.opened.clone();
        let mut unopened = state.unopened.clone();
        opened.push((pressure, time));
        unopened.remove(&vi);
        if ele {
            return (state.move_ele(vi, final_dest_idx, time, unopened, opened), false);
        }
        else {
            return (state.move_human(vi, final_dest_idx, time, unopened, opened), false);
        }
    }
}
fn iterate_states_elephant(g: &Graph::<Node, i32, Directed>, v: &HashMap<String, NodeIndex>, states: Vec<State>) -> (Vec<State>,Vec<Vec<(i32,i32)>>){
    let mut new_states: Vec<State> = vec![];
    let mut finished: Vec<Vec<(i32,i32)>> = vec![];
    
    for state in states {
        let plm_human = dijkstra(g, v[&state.pos], None, |_| 1);
        let plm_ele = dijkstra(g, v[&state.elephant], None, |_| 1);

        // No more moves to consider, this state is finished
        if state.unopened.len() == 0  || (state.time_e == 0 && state.time == 0) {
            finished.push(state.opened.clone());
        }

        //Only one move try and move both the elephant and the human, push to finished
        else if state.unopened.len() == 1 {
            let vi = state.unopened.iter().next().unwrap();
            let (ele_s, ele_done) = try_move(g, v, &state, vi.to_string(), &plm_ele, true);
            let (hum_s, hum_done) = try_move(g, v, &state, vi.to_string(), &plm_human, false);
            if ele_done {finished.push(ele_s.opened);} else {new_states.push(ele_s);}
            if hum_done {finished.push(hum_s.opened);} else {new_states.push(hum_s);}
        }

        // Elephant has exhaused its time, only move the human
        else if state.time_e == 0 {
            for vi in &state.unopened{
                let (hum_s, hum_done) = try_move(g, v, &state, vi.to_string(), &plm_human, false);
                if hum_done {finished.push(hum_s.opened);} else {new_states.push(hum_s);}
            }
        }
        // Human has exhausted its time, only move the elephant
        else if state.time == 0 {
            for vi in &state.unopened{
                let (ele_s, ele_done) = try_move(g, v, &state, vi.to_string(), &plm_ele, true);
                if ele_done {finished.push(ele_s.opened);} else {new_states.push(ele_s);}
            }
        }
    
        else {
            for pairs in state.unopened.clone().into_iter().combinations(2){
                let (v1, v2) = (pairs[0].clone(), pairs[1].clone());
                let (ele_s, ele_done) = try_move(g, v, &state, v1, &plm_ele, true);
                let (hum_s, hum_done) = try_move(g, v, &state, v2, &plm_human, false);
                if ele_done {finished.push(ele_s.opened);} else {new_states.push(ele_s);}
                if hum_done {finished.push(hum_s.opened);} else {new_states.push(hum_s);}
            }
        }
        
    }

    return (new_states, finished);
}
fn main() {
    let tunnels = fs::read_to_string("./src/input.txt").expect("Error reading file");
    let (g, v, non_zero) = create_graph(&tunnels);
    let start: String = "AA".to_string();
    let mut states: Vec<State> = vec![State::new(start.clone(), v[&start], 26, non_zero.clone(), vec![])];
    let mut end: Vec<Vec<(i32,i32)>> = vec![];
    let mut finished: Vec<Vec<(i32,i32)>> = vec![]; 
    let mut max_p = 0;
    while states.len() > 0{
        println!("{}", states.len());
        (states, finished) = iterate_states_elephant(&g,&v,states);
        for state in finished {
            let mut total = 0;
            for open in state{
                
                let open_t = open.1;
                total += open.0 * open_t;
            }
            if total > max_p{
                max_p = total;
            }
        }
    }

    println!("Pressure {:?}", max_p);
     
    
}
