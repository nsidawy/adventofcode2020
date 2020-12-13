use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let lines = read_lines(path);

    println!("Part 1: {}", get_batch_count(&lines));
    println!("Part 2: {}", get_batch_count_v2(&lines));
}

fn get_batch_count(lines: &Vec<String>) -> u32{
    let mut current_batch = Vec::new();
    let mut total = 0;
    for line in lines {
        if line.len() == 0 {
            total += count_batch(&current_batch);
            current_batch.clear();
        }
        else {
            current_batch.push(line);
        }
    }
    total += count_batch(&current_batch);

    total
}

fn count_batch(batch: &Vec<&str>) -> u32 {
    let mut yes_tally: [u32; 26] = [0; 26];
    for record in batch {
        for c in record.chars() {
            yes_tally[(c as usize) - 97] = 1;
        }
    }

    yes_tally.iter().sum()
}

fn get_batch_count_v2(lines: &Vec<String>) -> u32{
    let mut current_batch = Vec::new();
    let mut total = 0;
    for line in lines {
        if line.len() == 0 {
            total += count_batch_v2(&current_batch);
            current_batch.clear();
        }
        else {
            current_batch.push(line);
        }
    }
    total += count_batch_v2(&current_batch);

    total
}

fn count_batch_v2(batch: &Vec<&str>) -> u32 {
    let mut yes_tally: [u32; 26] = [0; 26];
    for record in batch {
        for c in record.chars() {
            yes_tally[(c as usize) - 97] += 1;
        }
    }

    yes_tally.iter().map(|c| if *c == batch.len() as u32 {1}else{0}).sum()
}

fn read_lines(filename: String) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.map(|line| line.unwrap()).collect()
}