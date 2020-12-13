use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let lines = read_lines(path);

    println!("Part 1: {}", traverse_slope2(&lines, 1, 3));

    let val1: i64 = traverse_slope2(&lines, 1, 1) as i64;
    let val2: i64 = traverse_slope2(&lines, 1, 3) as i64;
    let val3: i64 = traverse_slope2(&lines, 1, 5) as i64;
    let val4: i64 = traverse_slope2(&lines, 1, 7) as i64;
    let val5: i64 = traverse_slope2(&lines, 2, 1) as i64;

    println!("Part2: {}", val1*val2*val3*val4*val5);
}

fn traverse_slope2(
    lines: &Vec<String>,
    down: usize,
    right: usize
) -> i32{
    let height = lines.len();
    let width = lines[0].len();
    let mut count = 0;
    for i in (0..height).step_by(down) {
        let j = i / down * right % width;
        if lines[i].chars().nth(j).unwrap() == '#'{
            count = count + 1;
        }
    }

    println!("down: {}; right: {}; count: {}", down, right, count);
    count
}

fn read_lines(filename: String) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.map(|line| line.unwrap()).collect()
}