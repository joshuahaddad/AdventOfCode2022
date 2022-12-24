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
    unopened: Vec<String>
}

impl State {
    fn new(start: String, node: NodeIndex, t: i32, non_zero: Vec<String>, open: Vec<(i32,i32)>) -> State{
        State {
            pos: start.clone(),
            time: t,
            unopened: non_zero,
            opened: open
        }
    }
    
    fn mv(&self, mv: String, node: NodeIndex, t: i32, non_zero: Vec<String>, open: Vec<(i32,i32)>) -> State{
        State {
            pos: mv,
            time: t,
            unopened: non_zero,
            opened: open
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

fn update_state(initial: &State, to_str: String, to_idx: NodeIndex, t_spent: i32, pressure: i32) -> State{
    let mut opened = initial.opened.clone();
    let mut unopened = initial.unopened.clone();
    let t = initial.time - t_spent;
    opened.push((pressure, t));
    let index = unopened.iter().position(|x| *x == to_str).unwrap();
    unopened.remove(index);
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

fn get_pressure(opened: Vec<(i32, i32)>, max_p: i32) -> i32{
    let mut total = 0;
    for valve in opened{
        let open_t = valve.1;
        total += valve.0 * open_t;
    }

    if total > max_p{
        return total;
    }

    return max_p;
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
        let index = unopened.iter().position(|x| *x == vi).unwrap();
        unopened.remove(index);
        if ele {
            return (state.move_ele(vi, final_dest_idx, time, unopened, opened), false);
        }
        else {
            return (state.move_human(vi, final_dest_idx, time, unopened, opened), false);
        }
    }
}
fn iterate_states_elephant(g: &Graph::<Node, i32, Directed>, v: &HashMap<String, NodeIndex>, states: HashSet<State>) -> (HashSet<State>, i32){
    let mut new_states: HashSet<State> = HashSet::new();
    let mut finished: Vec<Vec<(i32,i32)>> = vec![];
    let mut max_p = 0;
    for state in states {
        let plm_human = dijkstra(g, v[&state.pos], None, |_| 1);
        let plm_ele = dijkstra(g, v[&state.elephant], None, |_| 1);

        // No more moves to consider, this state is finished
        if state.unopened.len() == 0  || (state.time_e == 0 && state.time == 0) {
            max_p = get_pressure(state.opened, max_p);
        }

        //Only one move try and move both the elephant and the human, push to finished
        else if state.unopened.len() == 1 {
            let vi = state.unopened.iter().next().unwrap();
            let (ele_s, ele_done) = try_move(g, v, &state, vi.to_string(), &plm_ele, true);
            let (hum_s, hum_done) = try_move(g, v, &state, vi.to_string(), &plm_human, false);
            if ele_done {max_p = get_pressure(ele_s.opened, max_p);} else {new_states.insert(ele_s);}
            if hum_done {max_p = get_pressure(hum_s.opened, max_p);} else {new_states.insert(hum_s);}
        }

        // Elephant has exhaused its time, only move the human
        else if state.time_e == 0 {
            for vi in &state.unopened{
                let (hum_s, hum_done) = try_move(g, v, &state, vi.to_string(), &plm_human, false);
                if hum_done {max_p = get_pressure(hum_s.opened, max_p);} else {new_states.insert(hum_s);}
            }
        }
        // Human has exhausted its time, only move the elephant
        else if state.time == 0 {
            for vi in &state.unopened{
                let (ele_s, ele_done) = try_move(g, v, &state, vi.to_string(), &plm_ele, true);
                if ele_done {max_p = get_pressure(ele_s.opened, max_p);} else {new_states.insert(ele_s);}
            }
        }
    
        else {
            for pairs in state.unopened.clone().into_iter().permutations(2){
                let (v1, v2) = (pairs[0].clone(), pairs[1].clone());
                let (ele_s, ele_done) = try_move(g, v, &state, v1, &plm_ele, true);
                let (hum_s, hum_done) = try_move(g, v, &state, v2, &plm_human, false);
                if ele_done {max_p = get_pressure(ele_s.opened, max_p);} else {new_states.insert(ele_s);}
                if hum_done {max_p = get_pressure(hum_s.opened, max_p);} else {new_states.insert(hum_s);}
            }
        }
        
    }

    return (new_states, max_p);
}
fn main() {
    let tunnels = fs::read_to_string("./src/input.txt").expect("Error reading file");
    let (g, v, non_zero) = create_graph(&tunnels);
    let start: String = "AA".to_string();
    let mut states: HashSet<State> = HashSet::new();
    states.insert(State::new(start.clone(), v[&start], 26, non_zero.clone(), vec![]));
    let mut iter_max_p = 0;
    let mut max_p = 0;
    while states.len() > 0{
        println!("{}", states.len());
        (states, iter_max_p) = iterate_states_elephant(&g,&v,states);
        println!("Pressure {:?}", iter_max_p);
        if iter_max_p > max_p{
            max_p = iter_max_p;
        }
    }

    println!("Pressure {:?}", max_p);
     
    
}
