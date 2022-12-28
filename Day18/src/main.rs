use std::iter::zip;
use std::fs;
use itertools::Itertools;

fn manhattan(c1: &Vec<i32>, c2: &Vec<i32>) -> i32{
    c1.iter().zip(c2.iter()).map(|x| (x.0-x.1).abs()).sum()
}

fn get_cubes(lst: String) -> Vec<Vec<i32>>{
    lst.lines()
    .map(
        |line| line.split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
    ).collect()
}

fn get_covered(cubes: Vec<Vec<i32>>) -> usize{
    6*cubes.len() - 2*cubes.iter().combinations(2).map(|x| if manhattan(&x[0], &x[1]) <= 1 {1} else {0}).sum::<usize>()
}

fn main() {
    let lst = fs::read_to_string("./src/input.txt").expect("Error Reading File");
    let cubes = get_cubes(lst);
    println!("{:?}", get_covered(cubes));

}
