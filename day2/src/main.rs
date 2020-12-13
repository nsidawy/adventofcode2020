use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut path = String::new(); 
    path.push_str(env::current_dir().unwrap().to_str().unwrap());
    path.push_str("\\input\\input.txt");
    let lines = read_lines(path);

    println!("Part 1: {}", count_valid_ps(&lines, &validate1));
    println!("Part 2: {}", count_valid_ps(&lines, &validate2));
}

fn count_valid_ps(lines: &Vec<String>, validate: &dyn Fn(&String) -> bool) -> i32 {
    let mut count = 0;
    for line in lines {
        if validate(line) {
            count = count + 1;
        }
    }    
    count
}

fn validate1(entry: &String) -> bool {
    let inputs: Vec<&str> = entry.split(' ').collect();
    let minmax: Vec<&str> = inputs[0].split('-').collect();
    let min = minmax[0].parse::<i32>().unwrap();
    let max = minmax[1].parse::<i32>().unwrap();
    let letter: char = inputs[1].chars().next().unwrap();
    let mut count = 0;

    for c in inputs[2].chars() {
        if c == letter {
            count = count + 1;
        }
    }

    count >= min && count <= max
}

fn validate2(entry: &String) -> bool {
    let inputs: Vec<&str> = entry.split(' ').collect();
    let pos: Vec<&str> = inputs[0].split('-').collect();
    let pos1 = pos[0].parse::<i32>().unwrap();
    let pos2 = pos[1].parse::<i32>().unwrap();
    let letter: char = inputs[1].chars().next().unwrap();
    let c1 = inputs[2].chars().nth((pos1-1) as usize);
    let c2 = inputs[2].chars().nth((pos2-1) as usize);

    (c1.is_some() && c1.unwrap() == letter)
        ^ (c2.is_some() && c2.unwrap() == letter)
}

fn read_lines(filename: String) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut l = Vec::new();

    for line in lines {
        l.push(line.unwrap());
    }    

    l
}