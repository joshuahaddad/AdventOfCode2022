use std::fs;

struct CPU {
    cycles: i32,
    x: i32,
    signal_str: Vec<i32>
}

impl CPU {
    fn new() -> CPU{
        CPU {
            cycles: 0,
            x: 1,
            signal_str: vec![]
        }
    }
}

fn execute_instrs(instructions: &String) -> i32{
    let mut cpu = CPU::new();
    let mut sum_s = 0;
    for line in instructions.lines() {
        //println!("{}", line);
        let instr = &line[0..4];
        
        let (add_instr, cycles) = match instr {
            "noop" => (false, 1),
            "addx" => (true, 2),
            _ => panic!()
        };

        for cycle in 0..cycles{
            cpu.cycles += 1;

            if cpu.cycles == 20 || cpu.cycles >= 60 && cpu.cycles % 40 == 20 {
                cpu.signal_str.push(cpu.x*cpu.cycles);
                sum_s += cpu.x*cpu.cycles;
                
            }

            if add_instr && cycle == 1 {
                let arg: i32 = line[5..line.len()].parse().unwrap_or(0);
                cpu.x += arg;
            }         
        }
    }
    
    println!("{}", sum_s);
    return sum_s;
}
fn main() {
    let instructions = fs::read_to_string("./src/input.txt").expect("Error reading file");
    execute_instrs(&instructions);
}
