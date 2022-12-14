use std::fs;

fn find_matching_paren(lst: &str, start: usize) -> usize{
    let mut counter = 1;
    let mut i: usize = 0;
    while(counter > 0) {
        i += 1;
        if lst.chars().nth(i+start).unwrap() == '['{
            counter += 1;
        }
        if lst.chars().nth(i+start).unwrap() == ']'{
            counter -= 1;
        }
    }

    return i+start;
}
fn eval_pair(l: &str, r: &str, il: usize, ir: usize, valid: bool) -> bool{
    //println!("{} {}", l, r);
    let mut val = valid;
    let mut lpos = il;
    let mut rpos = ir;
    let mut r = r.clone().to_string();
    let mut l = l.clone().to_string();
    if !valid {
        return false;
    }
    // Check if left list ran out of elem before right list
    if l.len() <= il && r.len() > ir {
        return true;
    }
    else if l.len() <= il && r.len() <= ir {
        return true;
    }
    else if l.len() > il && r.len() <= ir {
        return false;
    }


    let lc = l.chars().nth(il).unwrap();
    let rc = r.chars().nth(ir).unwrap();
    let mut l_list = lc == '[';
    let mut r_list = rc == '[';
    //println!("{} {}", lc, rc);

    // Conv right to list
    if l_list && !r_list {
        r.insert(ir+1, ']');
        r.insert(ir, '[');
        r_list = true;
    }

    // Conv left to list
    if !l_list && r_list {
        l.insert(il+1, ']');
        l.insert(il, '[');
        l_list = true;
    }

    // Comparing two lists
    if l_list && r_list {
        // Strip the parenthesis for the lists
        lpos = find_matching_paren(&l, il);
        rpos = find_matching_paren(&r, ir);

        let l = &l.to_string()[il+1..lpos];
        let r = &r.to_string()[ir+1..rpos];

        // Eval the new strings
        return eval_pair(&l,&r,0,0,val);
    }


    // Comparing two values
    if !l_list && !r_list {
        if lc > rc {
            return false;
        }
        if lc < rc {
            return true;
        }
        else if l.len() < 2 {
            return true;
        }
        else if r.len() < 2 {
            return true;
        }
        else if lc == rc{
            lpos = l.len();
            rpos = r.len();

            val = eval_pair(&l, &r, il+2, il+2, val);
        }
    }
    return eval_pair(&l, &r, lpos, rpos, val);
}
fn main() {
    let data = fs::read_to_string("./src/input.txt").expect("Failed to load");
    let mut pairs = vec!["";2];
    let mut s = 0;
    let mut pair = 0;
    for (i, line) in data.lines().enumerate(){    
        if line == ""  {
            pair += 1;
            let valid = eval_pair(pairs[0], pairs[1], 0, 0, true);
            if valid {
                s += pair;
            }
            println!("{:?} {:?} {}", pairs[0], pairs[1], valid);
            //break;
        }
        else {
            pairs[i%3] = line;
        }
    }

    let valid = eval_pair(pairs[0], pairs[1], 0, 0, true);
    println!("{:?} {:?} {}", pairs[0], pairs[1], valid);
    if valid {
        s += pair;
    }
    println!("{}", s);

}
