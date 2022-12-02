use std::fs;

fn main(){
    let data = fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    
    let mut total_cal = 0;

    // Since we are only looking for the top 3 we can store each val manually.
    // In the general top-n case a sorted array of totals or an array len(n) with shifted values could be used.
    let mut max_cal = 0;
    let mut second_best = 0;
    let mut third_best = 0;

    for line in data.lines(){

        // New line is denoted by a blank. If not a newline add the calorie to this current Elf's cals
        if line != ""{
            total_cal += line.parse::<i32>().unwrap();
        }
        
        if line == "" {

            // If the summed cals are greater than the current seen top cal, shift 1 to 2, 2 to 3 and replace 1 with the found val
            if total_cal > max_cal {
                third_best = second_best;
                second_best = max_cal;
                max_cal = total_cal;
            }

            // If the summed cals are greater than the second highest we only need to shift 2 to 3 and replace the second
            else if total_cal > second_best{
                third_best = second_best;
                second_best = total_cal;
            }
            
            // If the summed cals are greater than the third we can replace directly
            else if total_cal > third_best{
                third_best = total_cal;
            }

            // New elf on the next iteration so reset the counter
            total_cal = 0;
        }
    }

    // Print the top 3 values for problem 1 and for error checking, Print the sum of the top 3 for problem 2
    println!("{}, {}, {}", max_cal, second_best, third_best);
    println!("{}", max_cal+second_best+third_best);
}