#![allow(non_snake_case)]

use std::fs;
use std::collections::HashMap;

fn main(){
    let data = fs::read_to_string("./src/input.txt").expect("Should have been able to read the file");
    let mut score: i32 = 0;

    for line in data.lines(){
        let opponent = &line[0..1];
        let choice = &line[2..3];

        println!("{}, {}", opponent, choice);
        let mut choice_vals = HashMap::new();
        choice_vals.insert(String::from("X"), 1);
        choice_vals.insert(String::from("Y"), 2);
        choice_vals.insert(String::from("Z"), 3);

        // Win conditions X (rock) vs C (scissors), Y (paper) vs A (rock), Z (scissors) vs B (paper)
        let win: bool = choice == "X" && opponent == "C"  
            || choice == "Y" && opponent == "A"
            || choice == "Z" && opponent == "B";

        // Draw conditions X vs A, Y vs B, Z vs C
        let draw: bool = choice == "X" && opponent == "A"  
            || choice == "Y" && opponent == "B"
            || choice == "Z" && opponent == "C";
        
        
        
        score += 6*(win as i32) + 3*(draw as i32) + choice_vals.get(choice).copied().unwrap_or(0);
    }
    println!("{}", score);
}