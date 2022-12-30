use regex::Regex;
use std::collections::{HashMap,VecDeque};
use std::fs;
use trees::{Tree, Node};

fn p1(){
    let mut set_monkeys: HashMap<String, i64> = HashMap::new();

    // Unset Queue
    let mut unset: VecDeque<[String;4]> = VecDeque::new();
    let mut children: HashMap<String, [String;2]> = HashMap::new();
    let mut parents: HashMap<String, String> = HashMap::new();

    let re_num = Regex::new(r"([\D]+): ([\D]+) ([+\-*/]) ([\D]+)").unwrap();
    for line in fs::read_to_string("./src/input.txt").expect("Error").lines(){
        if re_num.is_match(line){ 
            for cap in re_num.captures_iter(line){
                unset.push_back([cap[1].to_string(),cap[2].to_string(), cap[4].to_string(), cap[3].to_string()]);
                children.insert(cap[1].to_string(), [cap[2].to_string(), cap[4].to_string()]);
                parents.insert(cap[2].to_string(), cap[1].to_string());
                parents.insert(cap[4].to_string(), cap[1].to_string());
            }
        }
        else {
            let monkey = &line[0..4].to_string();
            let val: i64 = line[6..line.len()].parse().unwrap();
            set_monkeys.insert(monkey.to_string(), val);
            
        }
    }

    while let Some(monkey) = unset.pop_front(){
        if set_monkeys.contains_key(&monkey[1]) && set_monkeys.contains_key(&monkey[2]){
            let v1 = set_monkeys.get(&monkey[1]).unwrap();
            let v2 = set_monkeys.get(&monkey[2]).unwrap();
        
            let res = match monkey[3].as_str(){
                "+" => {v1+v2},
                "-" => {v1-v2},
                "/" => {v1/v2},
                "*" => {v1*v2},
                _ => unreachable!()
            };
            set_monkeys.insert(monkey[0].clone(), res);
        }

        else {
            unset.push_back(monkey);
        }
    }
    println!("Problem 1 {}", set_monkeys.get(&"root".to_string()).unwrap());
}

fn gen_tree(tree: &mut Tree<String>, node_str: &String, children: HashMap<String, [String;2]>){

    if !children.contains_key(node_str){
        return;
    }
    let node_children = children.get(node_str).unwrap().clone();
    for child in node_children.into_iter(){
        let mut child_tree = Tree::new(child.clone());
        gen_tree(&mut child_tree, &child, children.clone());
        tree.push_back(child_tree);
    }
}

// Attempts to set any nodes that have two set children
fn set_unset_nodes(unset: &mut VecDeque<[String;4]>,  set: &mut HashMap<String, i64>){
    let max_iter = unset.len()*10;
    let mut i: usize = 0;
    while i < max_iter && unset.len()>0{
        let monkey = unset.pop_front().unwrap();
        i+=1;
        if set.contains_key(&monkey[1]) && set.contains_key(&monkey[2]){
            let v1 = set.get(&monkey[1]).unwrap();
            let v2 = set.get(&monkey[2]).unwrap();
        
            let res = match monkey[3].as_str(){
                "+" => {v1+v2},
                "-" => {v1-v2},
                "/" => {v1/v2},
                "*" => {v1*v2},
                _ => unreachable!()
            };
            set.insert(monkey[0].clone(), res);
        }

        else {
            unset.push_back(monkey);
        }
    }
}

fn get_hmn_val(node: &Node<String>, unset: &mut Vec<[String; 4]>, set: &mut HashMap<String, i64>, target: i64) -> i64{
    
    if node.data().clone() == "humn"{
        return target;
    }
    
    let info = unset.iter().find(|&x| x[0].clone() == node.data().clone()).unwrap().clone();
    let left = info[1].clone();
    let right = info[2].clone();

    if set.contains_key(&right){
        let v = set.get(&right).unwrap();
        let res = match info[3].as_str(){
            "+" => {target-v},
            "-" => {target+v},
            "/" => {target*v},
            "*" => {target/v},
            _ => unreachable!()
        };
        unset.remove(unset.iter().position(|x| x[0].clone() == info[0]).unwrap());
        set.insert(info[0].clone(), res);
        let left_node = node.iter().nth(0).unwrap();
        return get_hmn_val(left_node, unset, set, res)
    }

    else if set.contains_key(&left){
        
        let v = set.get(&left).unwrap();
        let res = match info[3].as_str(){
            "+" => {target-v},
            "-" => {v-target},
            "/" => {target/v},
            "*" => {target/v},
            _ => unreachable!()
        };
        unset.remove(unset.iter().position(|x| x[0].clone() == info[0]).unwrap());
        set.insert(info[0].clone(), res);
        let right_node = node.iter().nth(1).unwrap();
        return get_hmn_val(right_node, unset, set, res)
    }
    unreachable!()

}

fn p2(){

    let mut set_monkeys: HashMap<String, i64> = HashMap::new();
    let mut unset: VecDeque<[String;4]> = VecDeque::new();
    let mut children: HashMap<String, [String;2]> = HashMap::new();
    let re_num = Regex::new(r"([\D]+): ([\D]+) ([+\-*/]) ([\D]+)").unwrap();

    for line in fs::read_to_string("./src/input.txt").expect("Error").lines(){
        if re_num.is_match(line){ 
            for cap in re_num.captures_iter(line){
                if cap[1].to_string() != "root"{
                    unset.push_back([cap[1].to_string(),cap[2].to_string(), cap[4].to_string(), cap[3].to_string()]);
                }
                children.insert(cap[1].to_string(), [cap[2].to_string(), cap[4].to_string()]);
            }
        }
        else {
            let mut monkey = &line[0..4].to_string();

            if monkey == "humn"{continue;}

            let val: i64 = line[6..line.len()].parse().unwrap();
            set_monkeys.insert(monkey.clone(), val);
            
        }
    }

    // Set all possible nodes
    set_unset_nodes(&mut unset, &mut set_monkeys);

    // Get [non human path root, human path root]
    let roots = children.get(&"root".to_string())
                                                .map(|v| 
                                                    if set_monkeys.contains_key(&v[0]) {[v[0].clone(), v[1].clone()]} else {[v[1].clone(), v[0].clone()]}
                                                ).unwrap();

    //Generate the tree
    let mut humn_tree = Tree::new(roots[1].clone());                               
    gen_tree(&mut humn_tree, &roots[1], children.clone());

    // Get the target which is the non human path resultant
    let target = *set_monkeys.get(&roots[0]).unwrap();

    // Convert unset for ease of use
    let mut unset: Vec<[String; 4]> = Vec::from(unset);

    // Breadth first search the tree to calculate the value of the human node
    let hmn_val = get_hmn_val(humn_tree.root(), &mut unset, &mut set_monkeys, target);
    println!("Problem 2 {:?}", hmn_val);

}
fn main() {
    p1();
    p2();
}
