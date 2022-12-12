use std::fs;
use regex::Regex;
use std::collections::HashSet;
struct Monkey {
    id: usize,
    items: Vec<i64>,
    op: (char, String),
    test: (i64, usize, usize),
    inspections: i64 
}

impl Monkey{
    fn new(id: usize, items: Vec<i64>, op: (char, String), test: (i64,usize,usize)) -> Monkey{
        Monkey {
            id: id,
            items: items,
            op: op,
            test: test,
            inspections: 0
        }
    }
}

fn get_conditional_monkey(test: (i64, usize, usize), val: i64) -> usize{
    let res = if val % test.0 == 0 {test.1} else {test.2};
    return res;
}
fn apply_op(op: &(char, String), old: i64) -> i64 {
    
    let operand: i64 = op.1.parse().unwrap_or(old);
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
        let mut items: Vec<i64> = re2.find_iter(&cap[2]).filter_map(|num| num.as_str().parse().ok()).collect();
        let id: usize = cap[1].parse().unwrap();
        let op: (char,String) = (cap[3].parse().unwrap(), cap[4].to_string());
        let test: (i64,usize,usize) = (cap[5].parse().unwrap(), cap[6].parse().unwrap(), cap[7].parse().unwrap());   
        return Monkey::new(id, items, op, test);
    }

    panic!();
}

fn turn(mut monkeys: Vec<Monkey>, decrease_fac: i64, ring_mod: i64) -> Vec<Monkey>{
    
    for i in 0..monkeys.len(){
        let op = &monkeys[i].op.clone();
        let test = monkeys[i].test;
        let mut recipients: Vec<(usize, i64)> = vec![];
        let mut inspections = 0;
        for item in monkeys[i].items.drain(..) {
            // First item is inspected and worry level increases then divided by the modifier
            let new_worry: i64 = apply_op(op, item)/decrease_fac % ring_mod;
            inspections += 1;

            // Then monkey throws the item
            let recipient = get_conditional_monkey(test, new_worry);
            recipients.push((recipient, new_worry));
        }

        monkeys[i].inspections += inspections;

        for recipient in recipients {
            monkeys[recipient.0].items.push(recipient.1);
        }

    }
    return monkeys;
}

fn get_monkeys(data: &String) -> Vec<Monkey> {
    let mut text_block = "".to_string();
    let mut monkeys: Vec<Monkey> = vec![];

    for (i, line) in data.lines().enumerate(){
        text_block.push_str(line);
        if line.len() == 0 {
            monkeys.push(parse_lines(&text_block));
            text_block = "".to_string();
        }
    }
    monkeys.push(parse_lines(&text_block));

    return monkeys;
}

fn solve_problem(divisor: i64, turns: i64){
    let mut ring_mod = 1;
    let monkey_business = fs::read_to_string("./src/input.txt").expect("Error reading file");
    let mut monkeys: Vec<Monkey> = get_monkeys(&monkey_business);

    for monkey in &monkeys {
        ring_mod *= monkey.test.0;
    }

    for i in 0..turns {
        //println!("{}", i);
        monkeys = turn(monkeys, divisor, ring_mod);
    }

    let mut m1: i64 = 0;
    let mut m2: i64 = 0;
    for monkey in monkeys{
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
fn main() {
    solve_problem(3, 20);
    solve_problem(1, 10000);
}
