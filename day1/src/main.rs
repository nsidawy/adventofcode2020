use std::env;
use std::fs;

fn main() {
    let target = 2020;
    let ints = get_ordered_ints();

    part1(target, &ints);
    part2(target, &ints);

    let r2 = solve(target, 2, &ints, Vec::new()).unwrap();
    let r3 = solve(target, 3, &ints, Vec::new()).unwrap();
    println!("part 2: {}", r2);
    println!("part 3: {}", r3);
}

fn solve(target: i32, depth: i32, ints: &Vec<i32>, values: Vec<i32>) -> Option<i32>{
    let num_values = values.len();
    for i in 0..(ints.len()-1) {
        let mut new_values = vec![0;num_values+1];
        new_values[..values.len()].copy_from_slice(&values);
        new_values[values.len()] = ints[i];
        if num_values + 1 == depth as usize {
            let sum: i32 = new_values.iter().sum();
            if sum == target {
                let mut mul = 1;
                for v in new_values {
                    mul = mul * v;
                }
                return Some(mul)
            }
        }
        else {
            let result = solve(target, depth, ints, new_values);
            if result.is_some() {
                return result
            }
        }
    }
    return None
}

fn part1(target: i32, ints: &Vec<i32>) {
    println!("Executing part 1...");
    for i in 0..(ints.len()-1) {
        let val1 = ints[i];
        for k in ((i+1)..(ints.len()-1)).rev() {
            let val2 = ints[k];
            if val1 + val2 == target {
                println!("Found: {} {}", val1, val2);
                println!("Result: {}", val1 * val2);
            }
            else if val1 + val2 < target {
                break;
            }
        }
    }
}

fn part2(target: i32, ints: &Vec<i32>) {
    println!("Executing part 2...");
    for i in 0..(ints.len()-1) {
        let val1 = ints[i];
        for k in (i+1)..(ints.len()-1) {
            let val2 = ints[k];
            for j in (k+1)..(ints.len()-1) {
                let val3 = ints[j];
                if val1 + val2 + val3 == target {
                    println!("Found: {} {} {}", val1, val2, val3);
                    println!("Result: {}", val1 * val2 * val3);
                }
                else if val1 + val2 + val3 > target {
                    break;
                }
            }
        }
    }
}

fn get_ordered_ints() -> Vec<i32> {
    let mut path = String::new(); 
    path.push_str(env::current_dir().unwrap().to_str().unwrap());
    path.push_str("\\input\\input.txt");

    let lines = fs::read_to_string(path).unwrap();
    let mut ints = Vec::new();
    for line in lines.split("\n") {
        let parse = line.parse::<i32>();
        if parse.is_ok() {
            ints.push(parse.unwrap());
        }
    }

    ints.sort();
    ints
}