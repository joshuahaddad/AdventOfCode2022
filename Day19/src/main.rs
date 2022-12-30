use regex::Regex;
use core::time;
use std::fmt;
use std::collections::HashSet;
use std::cmp::max;
use std::fs;
use std::time::{Duration, Instant};

#[derive(Eq, Hash, PartialEq, Clone)]
struct Blueprint {
    ore_cost: [i32;3],
    clay_cost: [i32;3],
    obsidian_cost: [i32;3],
    geode_cost: [i32;3],
    bottleneck: Vec<i32>
}

impl Blueprint {
    fn new(ore: [i32;3], clay: [i32;3], obsidian: [i32;3], geode: [i32;3]) -> Blueprint{
        let bottleneck: Vec<i32> = ore.into_iter().zip(clay.into_iter())
                            .zip(obsidian.into_iter())
                            .zip(geode.into_iter())
                            .map(|(((c1, c2),c3),c4)|  [c1,c2,c3,c4].into_iter().max().unwrap())
                            .collect();
        Blueprint { 
            ore_cost: ore, 
            clay_cost: clay, 
            obsidian_cost: obsidian, 
            geode_cost: geode,
            bottleneck: bottleneck
        }
    }

    pub fn iter(&self) -> std::array::IntoIter<[i32; 3], 4>{
        [self.ore_cost, self.clay_cost, self.obsidian_cost, self.geode_cost].into_iter()
    }

    pub fn get_cost(&self, robots: &[i32;4]) -> [i32;3]{
        let costs: [[i32; 3]; 4] = [self.ore_cost, self.clay_cost, self.obsidian_cost, self.geode_cost];
        let mut total_cost = [0,0,0];
        for (i, n_robot) in robots.iter().enumerate(){
            for (mat, cost) in costs[i].iter().enumerate(){
                total_cost[mat] += cost*n_robot;
            }
        }
        total_cost
    }
}

impl fmt::Display for Blueprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Blueprint Costs:
                    Ore Robot Cost: {} Ore {} Clay {} Obsidian\n
                    Clay Robot Cost: {} Ore {} Clay {} Obsidian\n
                    Obsidian Robot Cost: {} Ore {} Clay {} Obsidian\n
                    Geode Robot Cost: {} Ore {} Clay {} Obsidian"
                    ,self.ore_cost[0], self.ore_cost[1], self.ore_cost[2]
                    ,self.clay_cost[0], self.clay_cost[1], self.clay_cost[2]
                    ,self.obsidian_cost[0], self.obsidian_cost[1], self.obsidian_cost[2]
                    ,self.geode_cost[0], self.geode_cost[1], self.geode_cost[2])
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct State {
    robots: [i32;4],
    mats: [i32;4],
    bp: Blueprint,
    time: i32
    
}

impl State {
    fn new(_robots: [i32;4], _mats: [i32;4], _bp: Blueprint, _time: i32) -> State{
        State{
            robots: _robots,
            mats: _mats,
            bp: _bp,
            time: _time
        }
    }

    fn check_buy(&self, cost: [i32;3]) -> bool{
        for (i, c) in cost.iter().enumerate(){
            if c > &self.mats[i]{
                return false;
            }
        }
        return true;
    }

    fn get_new_state(&self, robs: [i32;4], costs: [i32;3]) -> State{
        let prod = self.robots.clone();
        let mut curr_robs = self.robots.clone();

        for i in 0..4{
            curr_robs[i] += robs[i];
        }

        let mut curr_mats = self.mats.clone();
        for i in 0..3{
            curr_mats[i] += prod[i]-costs[i];
        }
        curr_mats[3] += prod[3];
 
        let t = self.time-1;

        State::new(curr_robs, curr_mats, self.bp.clone(), t)
    }
}

fn parse_blueprint(line: String) -> Blueprint {
    let re = Regex::new(r"[\D]+([\d]+)").unwrap();
    let nums: Vec<i32> = re.captures_iter(&line)
                .map(|cap| cap.get(1).unwrap().as_str().parse::<i32>().unwrap())
                .collect();

    // Calculate costs in format [ore, clay, obsidian]
    let ore = [nums[1], 0, 0];
    let clay = [nums[2], 0, 0];
    let obsidian = [nums[3], nums[4], 0];
    let geode = [nums[5], 0, nums[6]];
    
    return Blueprint::new(ore, clay, obsidian, geode);
}

fn get_max_buys(curr_state: &State) -> [i32;4]{
    let mut max_robots = [100;4];
    for (i, cost) in curr_state.bp.iter().enumerate(){
        max_robots[i] = cost.iter().enumerate().map(|(mat, c)| if *c != 0 {curr_state.mats[mat] / c} else {100}).min().unwrap();
    }
    max_robots
}

fn get_choices(curr_state: State, past_states: &mut HashSet<State>, curr_max: i32) -> Vec<State>{
    let mut new_states: Vec<State> = vec![];

    for (i, robs) in [[0,0,0,0], [1,0,0,0],[0,1,0,0],[0,0,1,0],[0,0,0,1]].into_iter().enumerate(){

        if i > 0 && i < 3 && curr_state.bp.bottleneck[i-1] == curr_state.robots[i-1]{
            continue;
        }
        
        

        let cost = curr_state.bp.get_cost(&robs);
        let new_state = curr_state.get_new_state(robs, cost);
                    
        if curr_state.check_buy(cost) && !past_states.contains(&new_state){
            //println!("{:?}", cost);
            past_states.insert(new_state.clone());
            new_states.push(new_state);
        }
    }

    //println!("new state {}", new_states.len());
    new_states
}

fn iter_states(state: State, past_states: &mut HashSet<State>, max_geode: i32) -> i32{
    
    //println!("Time {} Mats {:?} Robots {:?} Max Geo {}", state.time, state.mats, state.robots, max_geode);
    if state.time == 0{
        //println!("Max Geo {}", max(state.mats[3], max_geode));
        return max(state.mats[3], max_geode);
    }

    let mut mx_g = max_geode;

    // Check if this state can reach the max geodes if it only produces new geodes machines, stop searching this branch if it cant
    let time_remaining = state.time + 1;
    if ((time_remaining - 1)*time_remaining)/2 
        + state.mats[3] 
        + time_remaining 
        * state.robots[3] < max_geode{
             past_states.insert(state.clone());
             return max_geode;
       }
    
    for choice in get_choices(state, past_states, mx_g){
        let x = iter_states(choice.clone(), past_states, mx_g);
        mx_g = max(max_geode, x);
    }
    
    return mx_g;
}
fn get_bp_max(bp: Blueprint, dur: i32) -> i32{
    let init_state = State::new([1,0,0,0], [0;4], bp, dur);
    let mut past_states: HashSet<State> = HashSet::new();
    
    return iter_states(init_state, &mut past_states, 0);
}
fn main() {
    let mut p1 = 0;
    let p1_timer = Instant::now();
    for (i, line) in fs::read_to_string("./src/input.txt").expect("Error").lines().enumerate(){
        let bp = parse_blueprint(line.to_string());
        let start = Instant::now();
        let max_g = get_bp_max(bp, 24);
        let duration = start.elapsed();
        p1 += max_g*(i+1) as i32;
         println!("BP {} Max Geo {} Took {:?}", i+1, max_g, duration);
    }
    println!("Part 1 {} Solved in {:?}", p1, p1_timer.elapsed());

    let mut p2 = 1;
    let p2_timer = Instant::now();
    for (i, line) in fs::read_to_string("./src/input.txt").expect("Error").lines().enumerate(){
        let bp = parse_blueprint(line.to_string());
        println!("{:?}", bp.bottleneck);
        let start = Instant::now();
        let max_g = get_bp_max(bp, 32);
        let duration = start.elapsed();
        p2 *= max_g;
        println!("BP {} Max Geo {} Took {:?}", i+1, max_g, duration);

        if (i == 2){
            break;
        }
    }
    println!("Part 2 {} Solved in {:?}", p2, p2_timer.elapsed());
    
}
