use std::iter::zip;
use std::fs;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn manhattan(c1: &Vec<i32>, c2: &Vec<i32>) -> i32{
    c1.iter().zip(c2.iter()).map(|x| (x.0-x.1).abs()).sum()
}

fn get_bounds(cubes: &Vec<Vec<i32>>) -> (i32, i32, i32){
    let (mut max_x, mut max_y, mut max_z) = (0,0,0);
    for cube in cubes{
        if cube[0] > max_x{
            max_x = cube[0];
        }
        if cube[1] > max_y{
            max_y = cube[1];
        }
        if cube[2] > max_z{
            max_z = cube[2];
        }
    }
    return (max_x, max_y, max_z);
}

fn generate_env(x: i32, y: i32, z: i32) -> HashSet<Vec<i32>>{
    let mut set: HashSet<Vec<i32>> = HashSet::new();
    for xi in 0..x{ 
        for yi in 0..y{
            for zi in 0..z{
                set.insert(vec![xi,yi,zi]);
            }
        }
    }
    set
}

fn get_adj_cubes(cube: &Vec<i32>) -> HashSet<Vec<i32>>{
    let mut adjacent: HashSet<Vec<i32>> = HashSet::new();
    adjacent.insert(vec![cube[0]+1, cube[1], cube[2]]);
    adjacent.insert(vec![cube[0], cube[1]+1, cube[2]]);
    adjacent.insert(vec![cube[0], cube[1], cube[2]+1]);
    adjacent.insert(vec![cube[0]-1, cube[1], cube[2]]);
    adjacent.insert(vec![cube[0], cube[1]-1, cube[2]]);
    adjacent.insert(vec![cube[0], cube[1], cube[2]-1]);
    return adjacent;
}
fn check_dirs(cubes: &Vec<Vec<i32>>, coord: &Vec<i32>, bounds: (i32, i32, i32), path: &mut HashSet<Vec<i32>>) -> bool{
    let dirs = [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)];
    let mut enclosed = true;
    for dir in dirs {
        let mut dir_loc = vec![coord[0]+dir.0, coord[1]+dir.1, coord[2]+dir.2];
        if !path.contains(&dir_loc){
            enclosed = enclosed && check_dir(&cubes, &coord, dir, bounds, path)
        }
    }
      
    if enclosed {
        //path.insert(coord.clone());
        //println!("{:?}", path.len());
        return true;
    }
    //println!("false");

    return false;
}
fn check_dir(cubes: &Vec<Vec<i32>>, cube: &Vec<i32>, dir: (i32, i32, i32), max_dir: (i32, i32, i32), past_locs: &mut HashSet<Vec<i32>>) -> bool{
    let mut loc = cube.clone();
    past_locs.insert(loc.clone());
    loc = vec![loc[0]+dir.0, loc[1]+dir.1, loc[2]+dir.2];

    if(cubes.contains(&loc)){
        return true;
    }
    if loc[0] > max_dir.0 || loc[1] > max_dir.1 || loc[2] > max_dir.2 {
        return false;
    }
    else if loc[0] <= 0 || loc[1] <= 0 || loc[2] <= 0{
        return false;
    }
    //  println!("{:?}", loc);
    
    return check_dirs(&cubes, &loc, max_dir, past_locs);

}
fn get_enclosed_air(cubes: &Vec<Vec<i32>>) -> usize{
    let bounds = get_bounds(&cubes);
    let mut enclosed_air: HashSet<Vec<i32>> = HashSet::new();

    for cube in cubes{
        let adjacent_set = get_adj_cubes(cube);
        for coord in adjacent_set {
            if !cubes.contains(&coord){
                let mut path: HashSet<Vec<i32>> = enclosed_air.clone();
                if check_dirs(&cubes, &coord, bounds, &mut path){
                    //println!("{:?}", path);
                    for val in path{
                        enclosed_air.insert(val);
                    }
                }
            }
        }
    }
    let mut sa = 0;
    for cube in cubes{
        for air in &enclosed_air{
            if manhattan(cube, &air) == 1{
                sa +=1;
            }
        }
    }
    return sa;
}
fn get_cubes(lst: String) -> Vec<Vec<i32>>{
    lst.lines()
    .map(
        |line| line.split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
    ).collect()
}

fn get_covered(cubes: &Vec<Vec<i32>>) -> HashMap<usize, usize>{
    let mut covered: HashMap<usize, usize> = (0..cubes.len()).map(|x| (x, 6_usize)).collect();
    for i in 0..cubes.len(){
        for j in i+1..cubes.len(){
            if manhattan(&cubes[i], &cubes[j]) <= 1{
                *covered.get_mut(&i).unwrap() -= 1;
                *covered.get_mut(&j).unwrap() -= 1;
            }
        }
    }
    covered
}


fn solve(cubes: Vec<Vec<i32>>){
    let mut covered = get_covered(&cubes);
    println!("Problem 1 {}", covered.values().sum::<usize>());
    let sa = get_enclosed_air(&cubes);
    println!("Problem 2 {}", covered.values().sum::<usize>()-sa);
}

fn main() {
    let lst = fs::read_to_string("./src/input.txt").expect("Error Reading File");
    let cubes = get_cubes(lst);
    solve(cubes);

}
