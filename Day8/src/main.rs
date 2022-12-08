use std::fs;
use std::collections::HashSet;
use itertools::Either;


fn generate_forest(data: String) -> Vec<Vec<i32>>{
    let mut forest: Vec<Vec<i32>> = vec![];
    for (i, line) in data.lines().enumerate(){
        forest.push(vec![]);
        for tree in line.chars() {
            match tree.to_digit(10) {
                Some(val) => {forest[i].push(val as i32);}
                None => {}
            }
        }
    }

    return forest;
}
fn create_range(rev: bool, low: usize, high: usize) -> Either<impl Iterator<Item = usize>, impl Iterator<Item = usize>> {
    if !rev {
        Either::Left(low..high)
    } else {
        Either::Right((low..high).rev())
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn get_n_trees(forest: &Vec<Vec<i32>>, axis: i32, rev: bool, pos: (usize, usize)) -> i32{
    let (row, col) = pos;
    let mut n_trees = 0;
    let height = forest[row][col];

    // If axis = 0 then row is constant
    if axis == 0 {
        // If rev = true we are looking right to left and end at 0, else we end at the end of the vec
        let col_range = if rev {create_range(rev, 0, col)} else {create_range(rev, col+1, forest[0].len())};
        
        for col_idx in col_range {
            let curr_tree = forest[row][col_idx];
            if curr_tree < height {
                n_trees += 1;
            }
            else {
                n_trees += 1;
                break;
            }
        }
    } 
    // If axis = 1 then col is constant
    else {
        // If rev = true we are looking bottom to top and end at 0, else we end at the end of the vec
        let row_range = if rev {create_range(rev, 0, row)} else {create_range(rev, row+1, forest.len())};
        
        for row_idx in row_range {
            let curr_tree = forest[row_idx][col];
            if curr_tree < height {
                n_trees += 1;
            }
            else {
                n_trees += 1;
                break;
            }
        }
    }
    return n_trees;
}
fn update_axis(forest: &Vec<Vec<i32>>, axis: i32, rev: bool, viz: &mut HashSet<(i32, i32)>){
    /*  axis = 0 check rows axis = 1 check columns
        rev = false check normally, rev = true check backwards (ie for horizontal read right to left)
        viz = current set of visible trees, if already contained no need to recheck, mutable so no need for returns
    */
    let forest_dir = if axis == 0 {forest.clone()} else {transpose(forest.clone())};

    
    let mut i = 0;
    for dir in forest_dir{
        let n = dir.len();
        let first = if rev {n-1} else {0}; 
        let mut tallest = dir[first];
        
        if !rev {
            let tup = if axis == 0 {(i,0)} else {(0, i)};
            viz.insert(tup);
        } else {
            // Transpose (i,j) if we are looking at cols
            let tup = if axis == 0 {(i, (n-1) as i32)} else {((n-1) as i32, i)};
            
            viz.insert(tup);
        }
        let iter_range = if !rev {create_range(rev, 1, dir.len())} else {create_range(rev, 0, n-1)};
        for j in iter_range{       
            if dir[j] > tallest {

                // Transpose (i, j) if we are looking at cols
                let tup = if axis == 0 {(i,j as i32)} else {(j as i32, i)};

                // Insert coordinate of tree
                viz.insert(tup);
                tallest = dir[j];
            }
        }
        i+=1;    
    }
    
}

fn get_visibility(forest: Vec<Vec<i32>>) -> i32{
    let mut visible_trees: HashSet<(i32, i32)> = HashSet::<(i32, i32)>::new();
    update_axis(&forest, 1, false, &mut visible_trees);
    update_axis(&forest, 1, true, &mut visible_trees);
    update_axis(&forest, 0, false, &mut visible_trees);
    update_axis(&forest, 0, true, &mut visible_trees);

    println!("Problem 1 {}", visible_trees.len());
    return visible_trees.len() as i32;
}

fn get_scenic_score(forest: &Vec<Vec<i32>>, pos: (usize, usize)) -> i32{
    let mut prod = 1;
    for axis in 0..2{
        let forward = get_n_trees(&forest, axis, false, pos);
        let rev = get_n_trees(&forest, axis, true, pos);
        //println!("Found Forward {} Rev {} for Axis {}", forward, rev, axis);
        prod *= forward*rev;
    }
    
    //println!("Scenic Score: {}", prod);
    return prod;
}

fn find_highest_score(forest: &Vec<Vec<i32>>) -> i32 {
    let mut max_score = 0;
    for (col, rows) in forest.iter().enumerate(){
        for (row, _) in rows.iter().enumerate() {
            let score = get_scenic_score(&forest, (row, col));
            if score > max_score {
                max_score = score;
            }
        }
    }
    println!("Problem 2 {}", max_score);
    return max_score;
}
fn main() {
    let forest_data = fs::read_to_string("./src/input.txt").expect("Error while reading file");
    let forest = generate_forest(forest_data);
    find_highest_score(&forest);
    get_visibility(forest);
}

#[test]
fn prob1(){
    let forest_data = fs::read_to_string("./src/test.txt").expect("Error while reading file");
    let forest = generate_forest(forest_data);
    assert_eq!(get_visibility(forest), 21)
}

#[test]
fn prob2(){
    let forest_data = fs::read_to_string("./src/test.txt").expect("Error while reading file");
    let forest = generate_forest(forest_data);

    let first_tree = (1 as usize, 2 as usize);

    assert_eq!(get_n_trees(&forest, 0, false, first_tree), 2);
    assert_eq!(get_n_trees(&forest, 0, true, first_tree), 1);
    assert_eq!(get_n_trees(&forest, 1, false, first_tree), 2);
    assert_eq!(get_n_trees(&forest, 0, true, first_tree), 1);
    assert_eq!(get_scenic_score(&forest, first_tree), 4);

    let second_tree = (3 as usize, 2 as usize);
    assert_eq!(get_n_trees(&forest, 0, false, second_tree), 2);
    assert_eq!(get_n_trees(&forest, 0, true, second_tree), 2);
    assert_eq!(get_n_trees(&forest, 1, false, second_tree), 1);
    assert_eq!(get_n_trees(&forest, 0, true, second_tree), 2);
    assert_eq!(get_scenic_score(&forest, second_tree), 8);

    assert_eq!(find_highest_score(&forest), 8);
}