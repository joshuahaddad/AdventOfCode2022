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

    // for row in &forest {
    //     let mut s: String = "".to_string();
    //     for val in row {
    //         s = format!("{}{}",s,val);
    //     }ie
    //     println!("{}", s);
    // }

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
            // Transpose if we are looking at cols
            let tup = if axis == 0 {(i, (n-1) as i32)} else {((n-1) as i32, i)};
            
            viz.insert(tup);
        }
        let iter_range = if !rev {create_range(rev, 1, dir.len())} else {create_range(rev, 0, n-1)};
        for j in iter_range{       
            if dir[j] > tallest {

                // Transpose if we are looking at cols
                
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

    println!("Size of visible_trees {}", visible_trees.len());
    return visible_trees.len() as i32;
}

fn main() {
    let forest_data = fs::read_to_string("./src/input.txt").expect("Error while reading file");
    let forest = generate_forest(forest_data);
    get_visibility(forest);
}

#[test]
fn prob1(){
    let forest_data = fs::read_to_string("./src/test.txt").expect("Error while reading file");
    let forest = generate_forest(forest_data);
    assert_eq!(get_visibility(forest), 21)
}