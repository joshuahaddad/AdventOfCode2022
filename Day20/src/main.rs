use std::fs;
use std::collections::HashMap;
use std::cmp::{min, max};

fn main() {
    let mut vals: Vec<i64> = fs::read_to_string("./src/input.txt")
                                .expect("Error reading file")
                                .lines()
                                .map(|val| val.parse::<i64>().unwrap())
                                .collect();
    mix(&vals, 1);
    let decryption_key = 811589153;
    let p2_vals: Vec<i64> = vals.iter().map(|x| x*decryption_key).collect();
    mix(&p2_vals, 10);
}

fn mix(vals: &Vec<i64>, n_steps: i64){
    let mut new_idxs: Vec<usize> = (0..vals.len()).collect();
    let n = vals.len() as i64; 
    for _ in 0..n_steps
    {
        for (idx, &num) in vals.iter().enumerate() {
            // find mixed that corresponds to the number in nums
            let mixed_idx = new_idxs.iter().position(|&v| v == idx).unwrap() as i64;
            
            new_idxs.remove(mixed_idx as usize);
            let new_mixed_idx = (mixed_idx + num).rem_euclid(n-1) as usize;
            new_idxs.insert(new_mixed_idx, idx);
        }
    }

    let zero_idx = vals.iter().position(|&r| r == 0).unwrap();
    let changed_zero = new_idxs.iter().position(|&mix_num| mix_num == zero_idx).unwrap() as i64;
    let nums: i64 = [1000, 2000, 3000].iter().map(|x| {
        let moved_i = ((changed_zero+x) % (n)) as usize;
        let original_i = new_idxs[moved_i];
        vals[original_i]
    }).sum();
    println!("Res: {:?}", nums);
}