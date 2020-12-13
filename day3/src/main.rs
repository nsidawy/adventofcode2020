use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut path = String::new(); 
    path.push_str(env::current_dir().unwrap().to_str().unwrap());
    path.push_str("\\input\\input.txt");
    let lines = read_lines(path);
    let grid = construct_grid(&lines);

    println!("Part 1: {}", traverse_slope(&grid, 1, 3));
    println!("Part 1: {}", traverse_slope2(&lines, 1, 3));

    let val1: i64 = traverse_slope2(&lines, 1, 1) as i64;
    let val2: i64 = traverse_slope2(&lines, 1, 3) as i64;
    let val3: i64 = traverse_slope2(&lines, 1, 5) as i64;
    let val4: i64 = traverse_slope2(&lines, 1, 7) as i64;
    let val5: i64 = traverse_slope2(&lines, 2, 1) as i64;

    println!("Part2: {}", val1*val2*val3*val4*val5);
}

fn traverse_slope(
    grid: &Vec<Vec<bool>>,
    down: i32,
    right: i32
) -> i32 {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let height = grid.len();
    let width = grid[0].len();
    let mut count: i32 = 0;

    loop {
        if y >= height {
            return count
        }
        if grid[y][x % width] {
            count = count + 1;
        }
        x = x + right as usize;
        y = y + down as usize;
    }
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
        let j = i * right % width;
        if lines[i].chars().nth(j).unwrap() == '#'{
            count = count + 1;
        }
    }
    count
}

fn construct_grid(lines: &Vec<String>) -> Vec<Vec<bool>>{
    let height = lines.len();
    let width = lines[0].len();
    let mut grid = vec![vec![false; width]; height];

    for i in 0..height {
        for j in 0..width {
            if lines[i].chars().nth(j).unwrap() == '#'{
                grid[i][j] = true;
            }
        }
    }

    grid
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