#![allow(non_snake_case)]

use std::fs;
use std::collections::HashMap;

fn problem1(data: String){
    let mut score: i32 = 0;
    let mut choice_vals = HashMap::new();
    choice_vals.insert(String::from("X"), 1);
    choice_vals.insert(String::from("Y"), 2);
    choice_vals.insert(String::from("Z"), 3);
    
    for line in data.lines(){
        let opponent = &line[0..1];
        let choice = &line[2..3];

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
    println!("Problem 1 Solution: {}", score);
}

fn problem2(data: String){
    let mut score: i32 = 0;
    
    // Define modifiers for choice
    let mut choice_vals = HashMap::new();
    choice_vals.insert(String::from("A"), 1);
    choice_vals.insert(String::from("B"), 2);
    choice_vals.insert(String::from("C"), 3);

    // Define win conditions with <opponent choice, my choice> as elements of hashmap
    let mut win_con = HashMap::new();
    win_con.insert(String::from("A"), String::from("B"));
    win_con.insert(String::from("B"), String::from("C"));
    win_con.insert(String::from("C"), String::from("A"));

    // Swap key/val pairs in win_con to get the lose conditions
    let mut lose_con: HashMap<String, String> = HashMap::new();
    lose_con.insert(String::from("B"), String::from("A"));
    lose_con.insert(String::from("C"), String::from("B"));
    lose_con.insert(String::from("A"), String::from("C"));

    let mut choice:String;
    let null_str = String::from("");

    for line in data.lines(){
        let opponent = &line[0..1];
        let outcome = &line[2..3];
        

        // Draw, add 3 point draw value and modifier for opponent choice
        if outcome == "Y"{
            score += 3 + choice_vals.get(opponent).copied().unwrap_or(0);
        }

        // Win, find choice and add 6 point win value + modifier for choice
        else if outcome == "Z"{
            choice = win_con.get(opponent).unwrap_or(&null_str).to_string();
            score += 6 + choice_vals.get(&choice).copied().unwrap_or(0);
        }

        else{
            choice = lose_con.get(opponent).unwrap_or(&null_str).to_string();
            score += choice_vals.get(&choice).copied().unwrap_or(0);
        }
    }

    println!("Problem 2 Solution: {}", score)
}

fn main(){
    let data = fs::read_to_string("./src/input.txt").expect("Should have been able to read the file");

    problem1(data.clone());
    problem2(data);
}