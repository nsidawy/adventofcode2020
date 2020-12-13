use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let lines = read_lines(path);
    let mut all_seat_ids: Vec<u32> = lines.iter().map(|l| get_seat_id(&l, 7)).collect();

    let part1_max = all_seat_ids.iter().max().unwrap();
    println!("Part 1: {}", part1_max);

    all_seat_ids.sort();
    println!("Part 2: {}", get_my_seat_id(&all_seat_ids));
}

fn get_my_seat_id(seat_ids: &Vec<u32>) -> u32 {
    for i in 0..seat_ids.len()-1 {
        if seat_ids[i+1] - seat_ids[i] == 2 {
            return seat_ids[i] + 1
        }
    }

    0
}

fn get_seat_id(seat: &str, front_back_num: usize) -> u32 {
    let row = get_spot(&seat[..front_back_num]);
    let aisle = get_spot(&seat[front_back_num..]);
    let num_aisle = 2u32.pow((seat.len() - front_back_num) as u32);

    row * num_aisle + aisle 
}

fn get_spot(seat: &str) -> u32 {
    let max_space = 2u32.pow(seat.len() as u32);
    let space: Vec<u32> = (0u32..max_space).collect();

    get_spot_helper(seat, &space)
}

fn get_spot_helper(seat: &str, space: &[u32]) -> u32 {
    if seat.len() == 0 {
        return space[0] as u32
    }

    let c = seat.chars().next().unwrap();
    if c == 'F' || c == 'L' {
        get_spot_helper(&seat[1..], &space[..(space.len()/2)])
    }
    else {
        get_spot_helper(&seat[1..], &space[(space.len()/2)..])
    }
}

fn read_lines(filename: String) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.map(|line| line.unwrap()).collect()
}