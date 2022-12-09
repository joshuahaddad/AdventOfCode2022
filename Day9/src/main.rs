use std::collections::HashSet;
use std::fs;

// Return the direction and the number of steps (1, 0, 5) means move +x 5 steps
fn dir_to_coord(instr: &str) -> [i32; 3] {
    let dir = &instr[0..1];
    let steps: i32 = instr[2..instr.len()].parse().unwrap_or(0);

    // Define R as +x, L as -x, U as +y, D as -y
    match dir {
        "R" => [1, 0, steps],
        "L" => [-1, 0, steps],
        "U" => [0, 1, steps],
        "D" => [0, -1, steps],
        _ => panic!(),
    }
}

fn l2_norm(head: [i32; 2], tail: [i32; 2]) -> i32 {
    (head.iter()
        .zip(tail.iter())
        .map(|(h, t)| (h - t).pow(2))
        .sum::<i32>() as f32).sqrt() as i32
    
}

/* Checks if the tail is disjoint from the head by calculating the x,y distances:
    ..H
    .T. => true

    ..H
    ..T => false
*/
fn tail_disjoint(head: [i32; 2], tail: [i32; 2]) -> bool {
    head.iter()
        .zip(tail.iter())
        .map(|(h, t)| (h - t).abs())
        .sum::<i32>() > 1
}

/*
    Normal movement occurs when l2 norm is > 1
    Disjoint movement occurs when l1 norm is > 1
*/
fn problem1(instructions: &String) -> i32{
    let s = [0, 0];
    let mut head_pos = s;
    let mut tail_pos = s;
    let mut visited = HashSet::<(i32, i32)>::new();

    for instr in instructions.lines() {
        let dir = dir_to_coord(instr);

        for _ in 0..dir[2] {
            // println!("Current head: {:?} Current Tail: {:?}", head_pos, tail_pos);
            let disjoint = tail_disjoint(head_pos, tail_pos);
            head_pos[0] += dir[0];
            head_pos[1] += dir[1];

            let dist = l2_norm(head_pos, tail_pos);

            visited.insert((tail_pos[0], tail_pos[1]));

            // Head and tail were connected horizontally
            if !disjoint && dist > 1 {
                tail_pos[0] += dir[0];
                tail_pos[1] += dir[1];
            } 
            
            // Head and tail were connected diagonally
            else if disjoint && dist > 1 {

                match dir[0]{
                    1 => {tail_pos = [head_pos[0]-1, head_pos[1]];},
                    -1 => {tail_pos = [head_pos[0]+1, head_pos[1]];},
                    _ => ()
                }

                // If the head moved horizontally the tail is either left one or right one
                match dir[1]{
                    1 => {tail_pos = [head_pos[0], head_pos[1]-1];},
                    -1 => {tail_pos = [head_pos[0], head_pos[1]+1];},
                    _ => ()
                }
            }
            // println!("Final head: {:?} Final Tail: {:?}", head_pos, tail_pos);
            // If the tail has not visited the location insert into the set
            
        }

        visited.insert((tail_pos[0], tail_pos[1]));
        
        
    }
    println!("{}", visited.len());
    return visited.len() as i32;
}

fn main() {
    let instructions = fs::read_to_string("./src/input.txt").expect("Error reading file");
    problem1(&instructions);
}

#[test]
fn p1() {
    let instructions = fs::read_to_string("./src/test.txt").expect("Error reading file");
    assert_eq!(problem1(&instructions), 13);
}
