#![allow(non_snake_case)]
use std::fs;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn build_dir(data: &String) -> HashMap<String, (String, Vec<(String, i32)>)> {

    // Represent the directory as a hashmap mapping (filepath) -> (parent filepath, Vec<(file name, file size)>)
    let mut dir_tree: HashMap<String, (String, Vec<(String, i32)>)> = HashMap::new();
    let mut reading_dir: bool = false;
    let mut curr_dir: String = "~".to_string();
    let mut parent_dir: String = "".to_string();
    for line in data.lines(){
        
        if line.starts_with("$ cd"){
            // End ls command
            reading_dir = false;

            // Get the directory as an argument
            let args = &line[5..line.len()];

            // If we are moving up a directory use the hashmap to find the parent dir
            if args == ".."{
                let _dbg = curr_dir.clone();
                curr_dir = parent_dir.clone();
                parent_dir = dir_tree.get(&curr_dir).unwrap().0.clone();
                
                //println!("Moving Up from dir {} to dir {} with parent {}", _dbg, curr_dir, parent_dir);
            }

            // If we are moving into a directory
            else {
                parent_dir = curr_dir.clone();
                if args != "/"{
                    curr_dir.push_str(args);
                    curr_dir.push_str("/");
                } else {
                    curr_dir.push_str(args);
                }
                
                //println!("Moving From dir {} to dir {}", parent_dir, curr_dir);
                
            }
            
            // Initialize the hashmap entry if it doesnt exist
            if !dir_tree.contains_key(&curr_dir){
                let files = Vec::<(String, i32)>::new();
                dir_tree.insert(curr_dir.clone(), (parent_dir.clone(), files));
            }
            // println!("Reading {} as having parent dir {}", curr_dir, parent_dir);
        }
        else if line.starts_with("$ ls"){
            // Start an ls command
            reading_dir = true;
        }

        // This should function like an else but to be careful make independent if
        else if reading_dir && !line.starts_with("dir"){
            let space = line.find(" ").unwrap() as usize;
            let size: i32 = line[0..space].parse::<i32>().unwrap();
            let file = &line[space+1..line.len()];

            match dir_tree.entry(curr_dir.clone()) {
                Entry::Vacant(e) => {e.insert((parent_dir.clone(), vec![(file.to_string(), size)]));},
                Entry::Occupied(mut e) => { e.get_mut().1.push((file.to_string(), size)); }
            }
            //println!("File {} Size {}", file, size);
        }

    }

    return dir_tree;
}

fn get_size_dirs(dir_tree: HashMap<String, (String, Vec<(String, i32)>)>) -> HashMap<String, i32>{

    let mut dir_sizes: HashMap<String, i32> = HashMap::new();

    for (dir, value) in &dir_tree {
        for tup in &value.1 {
            // Add the size to the current inspected dir
            match dir_sizes.entry(dir.to_string()){
                Entry::Vacant(e) => {e.insert(tup.1);},
                Entry::Occupied(mut e) => {*e.get_mut() += tup.1;}
            }

            // Iterate up the parent_dirs and add the value
            let mut curr_parent = value.0.clone();
            while curr_parent != "~"{
                // Add file to parent dir size
                match dir_sizes.entry(curr_parent.to_string()){
                    Entry::Vacant(e) => {e.insert(tup.1);},
                    Entry::Occupied(mut e) => {*e.get_mut() += tup.1;}
                }

                // Move to next parent
                let new_parent = dir_tree.get(&curr_parent).unwrap();
                curr_parent = new_parent.0.clone();
            }
        }   
     }
    
    return dir_sizes;
}

fn problem1(data: &String) -> i32 {
    let dir_tree = build_dir(data);
    let dir_sizes = get_size_dirs(dir_tree);
    let mut s = 0;
    for (_dir, value) in &dir_sizes {
        if *value <= 100000 {
            s += value;
        }
    }
    return s;
}

fn problem2(data: &String) -> i32 {
    let dir_tree = build_dir(data);
    let dir_sizes = get_size_dirs(dir_tree);

    let DL_SIZE = 30000000;
    let CAPACITY = 70000000;
    let REQ_FREE = DL_SIZE - (CAPACITY - dir_sizes.get("~/").unwrap());
    let mut min_s = DL_SIZE;
    
    for (_dir, value) in &dir_sizes {
        if *value <= min_s &&  *value >= REQ_FREE{
            
            min_s = *value;
        }
    }
    return min_s;
}
fn main() {
    let data = fs::read_to_string("./src/input.txt").expect("Error while reading file");
    println!("{}", problem1(&data));
    println!("{}", problem2(&data));
}

#[test]
fn test_p1 (){
    let data = fs::read_to_string("./src/test.txt").expect("Error while reading file");
    assert_eq!(problem1(&data), 95437)
}

#[test]
fn test_p2 (){
    let data = fs::read_to_string("./src/test.txt").expect("Error while reading file");
    assert_eq!(problem2(&data), 24933642)
}
