use std::env;
use std::fs::File;
use std::io::{self, BufRead};

type Grid = Vec<Vec<Seat>>;

enum Direction {
    N,
    NW,
    W,
    SW,
    S,
    SE,
    E,
    NE,
}

impl Direction {
    pub fn get_next_pos(&self, (i, j): (isize, isize)) -> (isize, isize) {
        match &self {
            Direction::N => (i+1, j),
            Direction::NW => (i+1, j-1),
            Direction::NE => (i+1, j+1),
            Direction::E => (i, j+1),
            Direction::W => (i, j-1),
            Direction::SE => (i-1, j+1),
            Direction::S => (i-1, j),
            Direction::SW => (i-1, j-1),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
enum Seat {
    Floor,
    Empty,
    Filled,
}

fn main() {   
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let grid = read_lines(path);
    let result1 = get_fill_count(&calc(&grid, false, 4));
    let result2 = get_fill_count(&calc(&grid, true, 5));

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}

fn get_fill_count(grid: &Grid) -> usize {
    grid
        .iter()
        .map(|g| 
            g.iter().filter(|s| **s == Seat::Filled)
            .count()
        )
        .sum::<usize>()
}

fn calc(grid: &Grid, recurse: bool, filled_max: u16) -> Grid {
    let mut grid_next = grid.clone();
    let mut changed = false;
    let surrounding = [
        Direction::N,Direction::NE,Direction::NW,
        Direction::E,Direction::W,
        Direction::S,Direction::SE,Direction::SW,
    ];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let pos = (i as isize, j as isize);
            match grid[i][j] {
                Seat::Empty => 
                    if surrounding.iter().all(|d| is_next_empty(grid, pos, d, recurse)){
                       grid_next[i][j] = Seat::Filled; 
                       changed = true;
                    },
                Seat::Filled =>
                    if surrounding.iter().filter(|d| !is_next_empty(grid, pos, *d, recurse)).count() >= filled_max as usize {
                       grid_next[i][j] = Seat::Empty; 
                       changed = true;
                    }
                _ => ()
            }
        }
    }
    if changed {
        calc(&grid_next, recurse, filled_max)  
    }
    else {
        grid_next
    }
}

fn is_next_empty(grid: &Grid, (i, j): (isize, isize), direction: &Direction, recurse: bool) -> bool {
    let (i_next, j_next) = direction.get_next_pos((i, j));
    if i_next < 0 || i_next >= grid.len() as isize
        || j_next < 0 || j_next >= grid[0].len() as isize {
        return true
    }

    let next_seat = grid[i_next as usize][j_next as usize];
    match next_seat {
        Seat::Filled => false,
        Seat::Empty => true,
        Seat::Floor => 
            if recurse { 
                is_next_empty(&grid, (i_next, j_next), direction, true) 
            } else { 
                true 
            },
    }
}

fn read_lines(filename: String) -> Grid {
    let file = File::open(filename).unwrap();
    let lines: Vec<String> = io::BufReader::new(file).lines().map(|l| l.unwrap()).filter(|l| l.len() > 0).collect();
    let mut grid: Grid = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        grid.push(Vec::new());
        for c in line.chars() {
            grid[i].push(match c {
                '.' => Seat::Floor,
                '#' => Seat::Filled,
                'L' => Seat::Empty,
                _ => Seat::Empty
            });
        }
    }
    grid
}