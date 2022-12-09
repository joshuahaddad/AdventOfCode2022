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
fn sign(x: i32) -> i32{
    match x {
        x if x < 0 => -1,
        x if x > 0 => 1,
        _ => 0
    }
}

fn dbg_print(width: usize, height: usize, positions: Vec<[i32;2]>) {

    let mut lines = vec![vec!['.';width];height];
    for (i, pos) in positions.iter().enumerate() {
        let x = pos[0] as usize;
        let y = pos[1] as usize;
        lines[y][x] = char::from_digit(i as u32, 10).unwrap()
    }
    for line in lines.iter().rev() {
        let l: String = line.into_iter().collect();
        println!("{}", l);
    }
}
fn general_sol(instructions: &String, n_knots: usize, pos_start: [i32;2]) -> i32{

    // Initialize the head + knots
    let mut knot_pos = vec![pos_start;n_knots+1];
    let mut visited = HashSet::<(i32, i32)>::new();

    for instr in instructions.lines() {
        let dir = dir_to_coord(instr);
        
        for _ in 0..dir[2] {
            knot_pos[0] = [knot_pos[0][0] + dir[0], knot_pos[0][1] + dir[1]];

            for i in 1..n_knots+1{
                let dx = knot_pos[i-1][0] - knot_pos[i][0];
                let dy = knot_pos[i-1][1] - knot_pos[i][1];
                if dx.pow(2)+dy.pow(2) >= 4  {
                    knot_pos[i] = [knot_pos[i][0]+sign(dx), knot_pos[i][1]+sign(dy)];
                }
            }      
            visited.insert((knot_pos[n_knots][0], knot_pos[n_knots][1]));         
        }
        
    }
    println!("{}", visited.len());
    return visited.len() as i32;
}

fn main() {
    let instructions = fs::read_to_string("./src/input.txt").expect("Error reading file");
    general_sol(&instructions, 1, [0,0]);

    let instr = fs::read_to_string("./src/input.txt").expect("Error reading file");
    general_sol(&instr, 9, [0,0]);
}

#[test]
fn p1() {
    let instructions = fs::read_to_string("./src/test.txt").expect("Error reading file");
    assert_eq!(general_sol(&instructions, 1, [0,0]), 13);
}

#[test]
fn p2() {
    let inst1 = fs::read_to_string("./src/test.txt").expect("Error reading file");
    let inst2 = fs::read_to_string("./src/test2.txt").expect("Error reading file");
    assert_eq!(general_sol(&inst1, 7, [0,0]), 1);
    assert_eq!(general_sol(&inst2, 9, [11,5]), 36);
}
