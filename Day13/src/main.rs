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
    let mut s = 0;
    let mut pair = 0;
    for (i, line) in data.lines().enumerate(){    
        if line == ""  {
            pair += 1;
            if compare(&pairs[0], &pairs[1]) != Ordering::Greater {
                s += pair;
            } 
        }
        else {
            println!("{:?}", line);
            pairs[i%3] = json::parse(&line).unwrap();
        }
    }

    if compare(&pairs[0], &pairs[1]) != Ordering::Greater {
        s += pair;

    }
    println!("{}", s);

}
