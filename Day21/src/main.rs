use regex::Regex;
use std::collections::{HashMap,VecDeque};
use std::fs;

fn main() {
    let mut set_monkeys: HashMap<String, i64> = HashMap::new();

    // Unset Queue
    let mut unset: VecDeque<[String;4]> = VecDeque::new();

    let re_num = Regex::new(r"([\D]+): ([\D]+) ([+\-*/]) ([\D]+)").unwrap();
    for line in fs::read_to_string("./src/input.txt").expect("Error").lines(){
        if re_num.is_match(line){ 
            for cap in re_num.captures_iter(line){
                unset.push_back([cap[1].to_string(),cap[2].to_string(), cap[4].to_string(), cap[3].to_string()]);
            }
        }
        else {
            let mut monkey = &line[0..4].to_string();
            let val: i64 = line[6..line.len()].parse().unwrap();
            set_monkeys.insert(monkey.clone(), val);
            
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


    println!("Part 1 {:?}", set_monkeys.get(&"root".to_string()).unwrap());

    
}
