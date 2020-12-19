use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq, Copy)]
enum Cube {
    Inactive,
    Active
}

struct Pos {
    x: usize,
    y: usize,
    z: usize,
    w: usize
}

impl Cube {
    pub fn build(c: char) -> Option<Cube> {
        match c {
            '#' => Some(Cube::Active),
            '.' => Some(Cube::Inactive),
            _ => None,
        }
    }
}

fn main() {   
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let cycles = 6usize;
    let cubes = read_lines(path, cycles, 4);
    let cubes = step(cubes, cycles);
    let active_count = cubes.iter()
        .fold(0i32, |s1, c1| s1 + c1.iter()
            .fold(0i32, |s2, c2| s2 + c2.iter()
                .fold(0i32, |s3, c3| s3 + c3.iter().map(|c3| match c3 {Cube::Active => 1i32, Cube::Inactive => 0}).sum::<i32>())));

    println!("{:#?}", active_count);
}

fn step(cubes: Vec<Vec<Vec<Vec<Cube>>>>, cycles: usize) -> Vec<Vec<Vec<Vec<Cube>>>> {
    if cycles == 0 {
        return cubes;
    }
    let mut next_cubes = Vec::new();
    for w in 0..cubes.len() {
        next_cubes.push(Vec::new());
        let new_w_length = next_cubes.len()-1;
        for z in 0..cubes[0].len() {
            next_cubes[new_w_length].push(Vec::new());
            let new_z_length = next_cubes[new_w_length].len()-1;
            for y in 0..cubes[0][0].len() {
                next_cubes[new_w_length][new_z_length].push(Vec::new());
                let new_y_length = next_cubes[new_w_length][new_z_length].len()-1;
                for x in 0..cubes[0][0][0].len() {
                    next_cubes[new_w_length][new_z_length][new_y_length].push(get_new_state(&cubes, Pos{z, y, x, w}));
                }
            }
        }
    }
    step(next_cubes, cycles - 1)
}

fn get_new_state(cubes: &Vec<Vec<Vec<Vec<Cube>>>>, pos: Pos) -> Cube {
    let mut count = 0;
    for x in (pos.x as isize -1)..(pos.x as isize + 2) {
        for y in (pos.y as isize -1)..(pos.y as isize + 2) {
            for z in (pos.z as isize -1)..(pos.z as isize + 2) {
                for w in (pos.w as isize -1)..(pos.w as isize + 2) {
                    if (x == pos.x as isize && y == pos.y as isize && z == pos.z as isize && w == pos.w as isize) 
                        || x < 0 || y < 0 || z < 0 || w < 0
                        || x >= cubes[0][0][0].len() as isize
                        || y >= cubes[0][0].len() as isize
                        || z >= cubes[0].len() as isize
                        || w >= cubes.len() as isize
                    {
                        continue;
                    }
                    if cubes[w as usize][z as usize][y as usize][x as usize] == Cube::Active {
                        count += 1;
                    }
                }
            }
        }
    }

    match cubes[pos.w][pos.z][pos.y][pos.x] {
        Cube::Active => if count == 2 || count == 3 { Cube::Active } else { Cube::Inactive}
        Cube::Inactive => if count == 3 { Cube::Active } else { Cube::Inactive}
    }
}

fn read_lines(filename: String, cycles: usize, dimension: usize) -> Vec<Vec<Vec<Vec<Cube>>>> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let cube_lines: Vec<Vec<Cube>> = lines
        .map(|line| {
            let line = line.unwrap();
            let mut cube_line = vec![Cube::Inactive; line.len() + 2 * cycles];
            for (i, c) in line.chars().enumerate() {
                cube_line[i + cycles] = Cube::build(c).expect("cannot parse character");
            }
            cube_line
        })
        .collect();
    
    let mut cube_lines4 = vec![vec![vec![vec![Cube::Inactive; cube_lines[0].len()]; cube_lines.len() + cycles * 2]; cycles * 2 + 1]; cycles * 2 + 1];
    for i in 0..cube_lines.len() {
        cube_lines4[cycles][cycles][cycles + i] = cube_lines.get(i).unwrap().to_vec();
    }
    cube_lines4
}