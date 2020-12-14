use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {   
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let ints = read_lines(path);

    let part1_result = find_bad_value(&ints, 25);
    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", find_contiguous_sum(&ints, part1_result));
}

fn find_contiguous_sum(ints: &Vec<u64>, target: u64) -> u64 {
    for i in 2..ints.len()-1 {
        for j in 0..ints.len()-i+1 {
            let slice = &ints[j..j+i];
            let sum: u64 = slice.iter().sum::<u64>();
            if sum == target {
                return slice.iter().min().unwrap() + slice.iter().max().unwrap()
            }
        }
    }
    0
}

fn find_bad_value(ints: &Vec<u64>, lookback: usize) -> u64 {
    let mut result: u64 = 0;
    for i in lookback..ints.len(){
        match find_sum(&ints[i-lookback..i], ints[i]) {
            Some((_,_)) => continue,
            None => {result = ints[i as usize]; break}
        }
    }
    result
}

fn find_sum(ints: &[u64], target: u64) -> Option<(u64,u64)>{
    let len = ints.len() ;
    for i in 0..len {
        for j in (i+1)..len {
            if ints[i] + ints[j] == target {
                return Some((ints[i], ints[j]))
            }
        }
    }
    None
}

fn read_lines(filename: String) -> Vec<u64> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines
        .map(|line| line.unwrap().parse::<u64>())
        .filter(|d| d.is_ok())
        .map(|d| d.unwrap())
        .collect()
}