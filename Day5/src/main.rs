use std::fs;

fn init_stacks(stack_init: &String) -> Vec<Vec<char>>{
    let n_stacks: usize = (stack_init.lines().next().unwrap().len()+1)/4;
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(n_stacks);
    (0..stacks.capacity()).for_each(|_| stacks.push(Vec::<char>::default()));

    // Creates a stack by reading the lines in reverse
    for line in stack_init.lines().rev(){
        let line = line.to_string();
        for i in 0..n_stacks{
            let c = line.as_bytes()[(i*4+1) as usize];

            // Ignore whitespaces, [, and ]. Only accept chars A-Z
            if c >= 65 && c <=90 {
                stacks[i].push(c as char);
            }
        }
    }

    return stacks;
}

fn parse_instruction(instr: String) -> (i32, usize, usize){
    // Remove the initial "move "
    let instr = &instr[5..instr.len() as usize];

    // Get the number of moves
    let pos = instr.chars().position(|c| c == ' ').unwrap();
    let n: i32 = instr[0..pos as usize].parse().unwrap();

    // Remove the " from "
    let instr = &instr[pos+6..instr.len() as usize];

    // Get the separator marked by " to "
    let pos = instr.chars().position(|c| c == ' ').unwrap();
    let from: i32 = instr[0..pos as usize].parse().unwrap();
    let to: i32 = instr[pos+4..instr.len() as usize].parse().unwrap();

    return (n, (from-1) as usize, (to-1) as usize);
}

fn execute_prob1(instrs: &String, mut stacks: Vec<Vec<char>>) -> String{
    for instr in instrs.lines(){
        let (n, from, to) = parse_instruction(instr.to_string());
        for _ in 0..n {
            let c: char = stacks[from].pop().unwrap();
            stacks[to].push(c);
        }
    }
    
    return stacks.into_iter().map(|stack| *stack.last().unwrap()).collect::<String>();
}

fn execute_prob2(instrs: &String, mut stacks: Vec<Vec<char>>) -> String {
    for instr in instrs.lines(){
        let (n, from, to) = parse_instruction(instr.to_string());

        let mut moved_items: String = "".to_string();
        for _ in 0..n {
            moved_items.push(stacks[from].pop().unwrap());
        }

        for c in moved_items.chars().rev(){
            stacks[to].push(c);
        }
    }
    return stacks.into_iter().map(|stack| *stack.last().unwrap()).collect::<String>();
}

fn main() {
    let stack_init = fs::read_to_string("./src/stacks.txt").expect("Error while reading file");
    let instrs = fs::read_to_string("./src/instructions.txt").expect("Error while reading the file");
    let stacks = init_stacks(&stack_init);
    println!("Problem 1: {}", execute_prob1(&instrs, stacks));

    let stacks = init_stacks(&stack_init);
    println!("Problem 2: {}", execute_prob2(&instrs, stacks));

}

#[test]
fn problem1() {
    let stack_init = fs::read_to_string("./src/test_stacks.txt").expect("Error while reading file");
    let instrs = fs::read_to_string("./src/test_instr.txt").expect("Error while reading the file");
    let stacks = init_stacks(&stack_init);
    assert_eq!(execute_prob1(&instrs, stacks), "CMZ".to_string());
}

#[test]
fn problem2() {
    let stack_init = fs::read_to_string("./src/test_stacks.txt").expect("Error while reading file");
    let instrs = fs::read_to_string("./src/test_instr.txt").expect("Error while reading the file");
    let stacks = init_stacks(&stack_init);
    assert_eq!(execute_prob2(&instrs, stacks), "MCD".to_string());
}
