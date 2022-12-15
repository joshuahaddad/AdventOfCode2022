use json::{JsonValue, array};
use std::cmp::{Ordering, min};
use std::fs;

fn compare(l: &JsonValue, r: &JsonValue) -> Ordering {
    // if we are comparing a number
    if l.is_number() && r.is_number(){
        // Convert to integer and compare
        return l.as_i16().unwrap().cmp(&r.as_i16().unwrap());
    }

    // if we are comparing two lists
    else if l.is_array() && r.is_array() {
        // compare each value
        for il in 0..min(l.len(), r.len()){
            

            // Recursive compare the two elements while they are equal return result if they are not
            let comp = compare(&l[il], &r[il]);
            if comp != Ordering::Equal{
                return comp;
            }
            
            
        }

        // If equal check the length of the remaining elements
        return l.len().cmp(&r.len());
    }

    // if one of the elements is a list and the other is not convert then compare
    else if l.is_number() {
        return compare(&array!(l.as_i16().unwrap()), &r);
    }
    else if r.is_number(){
        return compare(&l ,&array!(r.as_i16().unwrap()));
    }
    return Ordering::Equal;
}

fn main() {
    let data = fs::read_to_string("./src/input.txt").expect("Failed to load");
    let mut pairs = vec![JsonValue::Null;2];
    let num_pairs: usize = (data.lines().count()+1)/3;

    let mut packets = Vec::new();

    let mut s = 0;
    for (i, line) in data.lines().enumerate(){    
        if line == ""  {
            if compare(&pairs[0], &pairs[1]) != Ordering::Greater {
                s += (i/3) as i32 + 1;
            } 
        }
        else {
            pairs[i%3] = json::parse(&line).unwrap();
            packets.push(json::parse(&line).unwrap());
            
        }
    }

    if compare(&pairs[0], &pairs[1]) != Ordering::Greater {
        s += num_pairs as i32;

    }
    println!("Problem 1 {}", s);

    // Insert spacer packets and sort
    packets.push(array![2]);
    packets.push(array![6]);
    packets.sort_by(|l,r| compare(l,r));
    let i1 = packets.iter().position(|r| r == &array![2]).unwrap()+1;
    let i2 = packets.iter().position(|r| r == &array![6]).unwrap()+1;
    println!("Problem 1 {}", i1*i2);


}
