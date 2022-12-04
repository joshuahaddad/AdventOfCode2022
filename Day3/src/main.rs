#![allow(non_snake_case)]

use std::fs;
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

fn find_similarity(line: &String) -> char{
    let start: usize = 0;
    let mid: usize = line.chars().count()/2 as usize;
    let end: usize = (line.chars().count()) as usize;
    let set1: HashSet<char> = line[start..mid].chars().collect();
    let set2: HashSet<char> = line[mid..end].chars().collect();
    for val in set1.intersection(&set2){
        return *val;
    }
    panic!("No intersection found?");
}

fn problem1(data: &String) -> i32{
    let mut s: i32 = 0;
    for line in data.lines(){
        s += get_priority(find_similarity(&line.to_string()));
    }
    println!("{}", s);
    return s;
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
fn problem2(data: &String) -> i32{
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
    return s;
}

fn main() {
    let data = fs::read_to_string("./src/input.txt").expect("Should have been able to read the file");
    //let data: String = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw".to_string();
    problem1(&data);
    problem2(&data);

}


#[test]
fn prob1() {
    let data: String = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw".to_string();
    assert_eq!(problem1(&data), 157);
}

#[test]
fn prob2(){
    let data: String = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw".to_string();
    assert_eq!(problem2(&data), 70);
}

