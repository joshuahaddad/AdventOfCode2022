#![allow(non_snake_case)]

use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn get_priority(similarity: char) -> i32{

    let char_int: i32 = similarity as i32;
    if char_int >= 97 {
        return char_int - 96;
    }

    else {
        return char_int - 38;
    }
}

fn find_similarity(line: &str) -> char{
    let mut n_first: HashMap<char, i32> = HashMap::new();
    let mut n_second: HashMap<char, i32> = HashMap::new();
    let start_second: usize = line.chars().count()/2 as usize;
    for i in 0..start_second{
        n_first.entry(line.to_string().as_bytes()[i] as char).or_insert(0);
        n_second.entry(line.to_string().as_bytes()[(start_second+i)] as char).or_insert(0);

        if n_first.contains_key(&(line.to_string().as_bytes()[(start_second+i)] as char)){
            return line.to_string().as_bytes()[(start_second+i)] as char
        }
        else if n_second.contains_key(&(line.to_string().as_bytes()[i] as char)){
            return line.to_string().as_bytes()[i] as char;
        }
    }

    return '1';
}

fn problem1(data: &String){
    let mut s: i32 = 0;
    for line in data.lines(){
        s += get_priority(find_similarity(line));
    }
    println!("{}", s);
}

fn get_badge(group: &mut Vec<String>) -> char{

    let set: HashSet<char> = group[0].chars().collect();
    let set2: HashSet<char> = group[1].chars().collect();
    let set3: HashSet<char> = group[2].chars().collect();
    let mut sets: Vec<HashSet<char>> = vec![set, set2, set3];

    let (intersection, others) = sets.split_at_mut(1);
    let intersection = &mut intersection[0];
    
    for other in others {
        intersection.retain(|e| other.contains(e));
    }

    return intersection.drain().next().unwrap();
}
fn problem2(data: &String){
    let mut i: i32 = 0;
    let mut s: i32 = 0;
    let mut group: Vec<String> = vec!["","",""].into_iter().map(|s| s.to_owned()).collect();
    for line in data.lines(){
        group[(i%3) as usize] = line.to_string();
        i+=1;
        if i % 3 == 0 {
            s += get_priority(get_badge(&mut group));
        }
    }
    println!("{}", s);
}

fn main() {
    let data = fs::read_to_string("./src/input.txt").expect("Should have been able to read the file");
    problem1(&data);
    problem2(&data);

}
