use std::fs;
use std::collections::HashSet;
use std::cmp::{max, min};

fn generate_rocks(scan: String) -> (HashSet<(usize, usize)>, usize){
    let mut rocks: HashSet<(usize, usize)> = HashSet::new();
    let mut max_y: usize = 0;
    for line in scan.lines(){
        let mut line_iter = line.split(" -> ");
        let mut start = line_iter.next().unwrap();
        
        for end in line_iter{
            let mut ranges: Vec<(usize, usize)> = start.split(",")
            .zip(end.split(","))
            .map(|(c1, c2)| (c1.parse::<usize>().unwrap(), c2.parse::<usize>().unwrap()))
            .map(|(c1, c2)| (min(c1, c2), max(c1, c2)))
            .collect();
            
            for x in ranges[0].0..ranges[0].1+1{
                for y in ranges[1].0..ranges[1].1+1 {
                    rocks.insert((x,y));

                    if y > max_y {
                        max_y = y;
                    }
                }
            }
            start = end;
        }
    }

    return (rocks, max_y);
}

fn drop_sand(rocks: &mut HashSet<(usize, usize)>, gen: (usize, usize), void: usize, prob2: bool) -> bool{
    
    let mut sand = (gen.0, gen.1);
    let mut rest = false;
    while !rest {
        // Check if the vertical position contains a rock
        if rocks.contains(&(sand.0, sand.1+1)){
            // Check if the down-left contains a rock, if it doesnt move the sand
            if !rocks.contains(&(sand.0-1, sand.1+1)){
                sand = (sand.0-1, sand.1+1);
            }
            // Check if the down-right contains a rock, if it doesnt move the sand
            else if !rocks.contains(&(sand.0+1, sand.1+1)){
                sand = (sand.0+1, sand.1+1);
            }

            // If neither the sand is at rest in its current position
            else {
                rest = true;
                rocks.insert(sand);

                // Problem 2 only, if the sand comes to rest at the generator we are done
                if sand == gen {
                    return false;
                }
            }
        }

        // If no rock then the sand should fall down
        else {
            if sand.1+1 >= void {
                
                // If problem 2 is enabled expand the floor and end
                if prob2 {
                    rocks.insert(sand);
                    rocks.insert((sand.0, void));
                    return true;
                }
                else {
                    return false;
                }                
            }
            sand = (sand.0, sand.1+1);
        }    
    }
    //println!("{:?}", sand);
    
    return rest;
}

fn p1() {
    let scan = fs::read_to_string("src/input.txt").expect("Error reading string");
    let (mut rocks, max_y) = generate_rocks(scan);
    let source: (usize, usize) = (500, 0);
    let mut n = 0;
    while drop_sand(&mut rocks, source, max_y, false){
        n += 1;
    }
    println!("Problem 1 {}", n);
}

fn p2(){
    let scan = fs::read_to_string("src/input.txt").expect("Error reading string");
    let (mut rocks, mut max_y) = generate_rocks(scan);
    let source: (usize, usize) = (500, 0);
    let mut n = 0;
    while drop_sand(&mut rocks, source, max_y+2, true){
        n += 1;
        
    }

    // Add one because the loop will stop when the sand ends at the gen
    println!("Problem 2 {:?}", n+1);
}
fn main() {
    p1();
    p2();
}
