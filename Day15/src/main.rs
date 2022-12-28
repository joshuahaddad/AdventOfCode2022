use std::fs;
use regex::Regex;
use std::collections::HashSet;
use std::cmp::{min,max};

fn p1(signals: &String, y: i32){
    let re = Regex::new(r"^[\D]*x=(-?[\d]*)[\D]*y=(-?[\d]*)[\D]*x=(-?[\d]*)[\D]*y=(-?[\d]*)?").unwrap();
    let mut points: HashSet<i32> = HashSet::new();
    let mut beac_in_row: HashSet<i32> = HashSet::new();
    for beacon in signals.lines(){
        let caps = re.captures(beacon).unwrap();

        // Sensor coords
        let x1 = caps.get(1).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
        let y1 = caps.get(2).map_or(0, |m| m.as_str().parse::<i32>().unwrap());

        // Beacon coords
        let x2 = caps.get(3).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
        let y2 = caps.get(4).map_or(0, |m| m.as_str().parse::<i32>().unwrap());

        if y2 == y{
            beac_in_row.insert(x2);
        }

        let dist = (x1-x2).abs()+(y1-y2).abs();
        println!("({}, {}) ({}, {}) = {}", x1, y1, x2, y2, dist);

        // Get the y distance wrt the sensor
        let y_off = (y-y1).abs();
        if (dist - y_off) >= 0 {
            for x_off in 0..(dist - y_off)+1{
                points.insert(x1 + x_off as i32);
                points.insert(x1 - x_off as i32);
            }
        }
        
    }
    //println!("{:?}", points);
    println!("{:?}", points.len()-beac_in_row.len());
}

fn p2(signals: &String, y: i32){
    let re = Regex::new(r"^[\D]*x=(-?[\d]*)[\D]*y=(-?[\d]*)[\D]*x=(-?[\d]*)[\D]*y=(-?[\d]*)?").unwrap();
    let mut ranges: Vec<(i32,i32)> = vec![];

    // Generate ranges for this row
    for beacon in signals.lines(){
        let caps = re.captures(beacon).unwrap();

        // Sensor coords
        let x1 = caps.get(1).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
        let y1 = caps.get(2).map_or(0, |m| m.as_str().parse::<i32>().unwrap());

        // Beacon coords
        let x2 = caps.get(3).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
        let y2 = caps.get(4).map_or(0, |m| m.as_str().parse::<i32>().unwrap());

        let dist = (x1-x2).abs()+(y1-y2).abs();
        //println!("({}, {}) ({}, {}) = {}", x1, y1, x2, y2, dist);

        // Get the y distance wrt the sensor
        let y_off = (y-y1).abs();
        let x_dist = dist - y_off;
        if x_dist >= 0 {
            let low_x = x1-x_dist;
            let high_x = x1+x_dist;
            let mut curr_range = (low_x, high_x);
            let mut overlaps: Vec<usize> = vec![];

            for (i, range) in ranges.clone().iter().enumerate() {
                let (lb1, ub1) = *range;
                let (lb2, ub2) = curr_range;
                let full_contain = (lb1 <= lb2 && ub1 >= ub2) || (lb2 <= lb1 && ub2 >= ub1);
                let half_contain = (lb1 <= ub2 && lb2 <= lb1) || (lb2 <= ub1 && lb1 <= lb2);
                // Full contained move to next range
                if full_contain || half_contain{
                    curr_range = (min(lb1, lb2), max(ub1, ub2));
                    overlaps.push(i);
                }

            } 

            if overlaps.len()==0 {
                ranges.push(curr_range);
            }
            else {
                for overlap in overlaps.iter().rev(){
                    ranges.remove(*overlap);
                }
                ranges.push(curr_range);
            }
        }  
    }
    if (ranges.len() == 2){
        ranges.sort_by_key(|k| k.0);
        let x = (ranges[0].1+1) as i64;
        println!("{}", x*4_000_000 + y as i64);
    }
    
}

fn main() {
    let signals = fs::read_to_string("./src/input.txt").expect("Error reading file");
    p1(&signals, 2000000);
    for y in (0..4000000).rev(){
        if y % 10000 == 0 {
            println!("{}", y);
        }
        p2(&signals, y);
    }
    
}
