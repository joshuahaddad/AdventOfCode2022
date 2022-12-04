#![allow(non_snake_case)]

use std::fs;
use std::collections::HashSet;

fn generate_set(range: &String) -> HashSet<i32>{

    // Extract the numbers from [\d]*-[\d]* IE 88-101 -> 88 and 101 vars
    let separator: usize = range.chars().position(|c| c == '-').unwrap();
    let from: i32 = (&range[0..separator]).to_string().parse().unwrap();
    let to: i32 = (&range[separator+1..range.len() as usize]).to_string().parse().unwrap();

    // Create a set which contains all elements within the range
    return HashSet::from_iter(from..to+1);
}

fn solve_problems(data: &String) -> (i32, i32){

    // Problem 1 = n_full_overlaps and Problem 2 = n_overlap
    let mut n_full_overlaps: i32 = 0;
    let mut n_overlap: i32 = 0;

    for line in data.lines(){

        //String parsing convert ([\d]*-[\d]*,[\d]*-[\d]*) to two variables with form [\d]*-[\d]* IE 1-2,3-4 is converted to 1-2 and 3-4
        let cast_line = line.to_string();
        let separator: usize = line.chars().position(|c| c == ',').unwrap();
        let first_range: String = (&cast_line[0..separator]).to_string();
        let second_range: String = (&cast_line[separator+1..cast_line.len() as usize]).to_string();
        
        // Generate sets for the string ranges inclusive IE (1-3) -> HashSet(1,2,3)
        let set1: HashSet<i32> = generate_set(&first_range);
        let set2: HashSet<i32> = generate_set(&second_range);

        // If either set is a full subset of the other it is a complete overlap by problem 1 standard
        if set1.is_subset(&set2) || set2.is_subset(&set1){
            n_full_overlaps+=1;
        }

        // If the sets are not disjoint they share an element -> overlap by problem 2 standard
        if !set1.is_disjoint(&set2){
            n_overlap += 1;
        }
    }
    println!("{}", n_full_overlaps);
    println!("{}", n_overlap);
    return (n_full_overlaps, n_overlap);
}


fn main() {
    let data = fs::read_to_string("./src/input.txt").expect("Error while reading file");
    solve_problems(&data);
}

#[test]
fn problems() {
    let data: String = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8".to_string();
    let (prob1, prob2) = solve_problems(&data);
    assert_eq!(prob1, 2);
    assert_eq!(prob2, 4);
}