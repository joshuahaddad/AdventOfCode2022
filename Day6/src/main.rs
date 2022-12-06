use std::fs;

fn get_start_of_packet(buffer: &String, n_chars: usize) -> usize{
    let mut curr_str: String = "".to_string();
    for (val, c) in buffer.chars().enumerate(){
        for (i, c_str) in curr_str.clone().chars().enumerate(){
            if c == c_str {
                // Slice everything up to and including the repeated char's first occurence
                curr_str = curr_str[i+1..curr_str.len()].to_string();
                break;
            }
        }
        curr_str.push(c);
        if curr_str.len() == n_chars {
            // Indexed at 1 in the problem so add one
            return val+1;
        }
    }
    return 0 as usize;
}

// Grabbing some code from @Bryson14 https://github.com/Bryson14/RustLangFun/blob/main/advent_of_code_2022/src/day6.rs to check benchmarks for different implementations    
fn hashset_iter_check(s: &String, no_repeat_len: usize) -> usize {
    use std::collections::HashSet;
    let mut marker = no_repeat_len;
    for i in no_repeat_len..s.len() {
        let mut set: HashSet<char> = HashSet::new();
        s[i - no_repeat_len..i].chars().for_each(|c| {
            let _ = set.insert(c);
        });

        // if the hashset has all elements and there were no repeated characters,
        // then all elements were
        if set.len() < no_repeat_len {
            marker += 1;
        } else {
            return marker;
        }
    }
    marker
}

fn benchmark(func: &dyn Fn(&String, usize) -> usize, iterations: u32, buffer: usize, data: &str) {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    for _ in 0..iterations {
        let _ = func(&data.to_string(), buffer);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.3?}", elapsed);
    println!("Time per call: {:.4?}", elapsed / iterations)
}

fn main() {
    let buffer = fs::read_to_string("./src/input.txt").expect("Error while reading file");

    benchmark(&hashset_iter_check, 1000, 4, &buffer);
    benchmark(&get_start_of_packet, 1000, 4, &buffer);
    //let i = get_start_of_packet(&buffer, 4 as usize);
    //let j = get_start_of_packet(&buffer, 14 as usize);
    //println!("Problem 1 {} \nProblem 2 {}", i, j);
}

#[test]
fn prob1() {
    let data = fs::read_to_string("./src/test.txt").expect("Error while reading file");
    let mut buffers = data.lines();
    let n_chars: usize = 4;
    assert_eq!(get_start_of_packet(&buffers.next().unwrap().to_string(), n_chars), 7 as usize);
    assert_eq!(get_start_of_packet(&buffers.next().unwrap().to_string(), n_chars), 5 as usize);
    assert_eq!(get_start_of_packet(&buffers.next().unwrap().to_string(), n_chars), 6 as usize);
    assert_eq!(get_start_of_packet(&buffers.next().unwrap().to_string(), n_chars), 10 as usize);
    assert_eq!(get_start_of_packet(&buffers.next().unwrap().to_string(), n_chars), 11 as usize);

}

#[test]
fn prob2() {
    let data = fs::read_to_string("./src/test.txt").expect("Error while reading file");
    let mut buffers = data.lines();
    let n_chars: usize = 14;
    assert_eq!(get_start_of_packet(&buffers.next().unwrap().to_string(), n_chars), 19 as usize);
    assert_eq!(get_start_of_packet(&buffers.next().unwrap().to_string(), n_chars), 23 as usize);
    assert_eq!(get_start_of_packet(&buffers.next().unwrap().to_string(), n_chars), 23 as usize);
    assert_eq!(get_start_of_packet(&buffers.next().unwrap().to_string(), n_chars), 29 as usize);
    assert_eq!(get_start_of_packet(&buffers.next().unwrap().to_string(), n_chars), 26 as usize);

}