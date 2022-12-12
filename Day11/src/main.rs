use std::fs;
use regex::Regex;
use std::collections::HashSet;
struct Monkey {
    id: usize,
    items: Vec<u128>,
    op: (char, String),
    test: (u128, usize, usize),
    inspections: u128 
}

impl Monkey{
    fn new(id: usize, items: Vec<u128>, op: (char, String), test: (u128,usize,usize)) -> Monkey{

        Monkey {
            id: id,
            items: items,
            op: op,
            test: test,
            inspections: 0
        }
    }
}

fn get_conditional_monkey(test: (u128, usize, usize), val: u128) -> usize{
    let res = if val % test.0 == 0 {test.1} else {test.2};
    return res;
}
fn apply_op(op: &(char, String), old: u128) -> u128 {
    
    let operand: u128 = op.1.parse().unwrap_or(old);
    if op.0 == '+' {
        return old + operand
    }
    else if op.0 == '*' {
        return old * operand;
    }
    panic!();
}

fn parse_lines(text_block: &String) -> Monkey{
    let re = Regex::new(r"^[\D]*(\d*)[\D]*([\d, ]*)  [\D]*([*+]) ([old\d]*)[\D]*([\d]*)[\D]*([\d]*)[\D]*([\d]*)$").unwrap();
    let re2 = Regex::new(r"\d+").unwrap();
    for cap in re.captures_iter(&text_block){
        let mut items: Vec<u128> = re2.find_iter(&cap[2]).filter_map(|num| num.as_str().parse().ok()).collect();
        let id: usize = cap[1].parse().unwrap();
        let op: (char,String) = (cap[3].parse().unwrap(), cap[4].to_string());
        let test: (u128,usize,usize) = (cap[5].parse().unwrap(), cap[6].parse().unwrap(), cap[7].parse().unwrap());   
        return Monkey::new(id, items, op, test);
    }

    return Monkey::new(0, vec![0], ('a', "".to_string()), (0,0,0))
}

fn turn(mut monkeys: Vec<Monkey>, decrease_fac: u128, ring_mod: u128) -> Vec<Monkey>{
    
    for i in 0..monkeys.len(){
        let op = &monkeys[i].op.clone();
        let test = monkeys[i].test;
        let mut recipients: Vec<(usize, u128)> = vec![];
        let mut inspections = 0;
        for item in monkeys[i].items.drain(..) {
            // First item is inspected and worry level increases then divided
            let new_worry: u128 = apply_op(op, item)/decrease_fac % ring_mod;
            inspections += 1;

            // Then monkey throws the item
            let recipient = get_conditional_monkey(test, new_worry);
            recipients.push((recipient, new_worry));
            //monkeys[recipient].items.push(new_worry);
        }

        monkeys[i].inspections += inspections;

        for recipient in recipients {
            monkeys[recipient.0].items.push(recipient.1);
        }

    }
    return monkeys;
}

fn main() {
    let monkey_business = fs::read_to_string("./src/input.txt").expect("Error reading file");
    let mut monkeys: Vec<Monkey> = Vec::<Monkey>::new();
    let mut text_block = "".to_string();
    
    for (i, line) in monkey_business.lines().enumerate(){
        text_block.push_str(line);
        if line.len() == 0 {
            println!("{} {}", i, text_block);
            monkeys.push(parse_lines(&text_block));
            text_block = "".to_string();
        }
    }
    monkeys.push(parse_lines(&text_block));

    let mut ring_mod = 1;
    for monkey in &monkeys {
        ring_mod *= monkey.test.0;
        println!("Ring mod {} {}", monkey.test.0, ring_mod);
    }
    
    for i in 0..10000 {
        //println!("{}", i);
        monkeys = turn(monkeys, 1, ring_mod);
    }

    let mut m1: u128 = 0;
    let mut m2: u128 = 0;
    for monkey in &monkeys{
        if monkey.inspections > m1 {
            m2 = m1;
            m1 = monkey.inspections;
        }
        else if monkey.inspections > m2 {
            m2 = monkey.inspections;
        }
    }

    println!("m1 {} m2 {} prod {}", m1, m2, m1*m2);
}
