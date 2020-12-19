use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use chrono::prelude::*;

#[derive(Debug, Clone, PartialEq, Copy)]
enum Cube {
    Inactive,
    Active
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
    let dimension = 3usize;
    let (cubes, dimensions) = read_lines(path, cycles, dimension);
    let cubes = step(cubes, dimensions, cycles);
    let active_count = cubes.iter()
        .fold(0i32, |s, c| s + match c {Cube::Active => 1i32, Cube::Inactive => 0});

    println!("{:#?}", active_count);
}

fn step(cubes: Vec<Cube>, dimensions: Vec<usize>, cycles: usize) -> Vec<Cube> {
    if cycles == 0 {
        return cubes;
    }
    println!("cycle {} {:?}", cycles, Local::now().format("%H:%M:%S").to_string());
    let mut next_cubes = vec![Cube::Inactive; cubes.len()];
    for i in 0..cubes.len() {
        next_cubes[i] = get_new_state(&cubes, &dimensions, i)
    }
    step(next_cubes, dimensions, cycles - 1)
}

fn get_new_state(cubes: &Vec<Cube>, dimensions: &Vec<usize>, index: usize) -> Cube {
    let indices = break_cube_index(index, &dimensions);

    let neighbor_coords:Vec<Vec<usize>> = indices.iter().enumerate().map(|(i, ind)| {
        let mut v = Vec::new();
        if *ind as isize - 1 >= 0 {
            v.push(ind - 1);
        }
        if *ind + 1 < dimensions[i] {
            v.push(ind + 1);
        }
        v.push(*ind);
        v
    }).collect();
    let count = get_neighbors(&neighbor_coords, &mut Vec::new()).iter()
        .map(|n| get_cube_index(n, dimensions))
        .filter(|i| *i != index)
        .fold(0u32, |s, i| s + match cubes[i] {Cube::Active => 1, Cube::Inactive => 0});
    match cubes[index] {
        Cube::Active => if count == 2 || count == 3 { Cube::Active } else { Cube::Inactive}
        Cube::Inactive => if count == 3 { Cube::Active } else { Cube::Inactive}
    }
}

fn get_neighbors(neighbor_coords: &Vec<Vec<usize>>, cur: &mut Vec<usize>) -> Vec<Vec<usize>> {
    if neighbor_coords.len() == 0 {
        return vec!(cur.to_vec());
    }
    let mut result = Vec::new();
    let next_neighbor_coords: Vec<Vec<usize>> = neighbor_coords[1..].to_vec();
    for n in neighbor_coords[0].iter() {
        cur.push(*n);
        result.push(get_neighbors(&next_neighbor_coords, cur));
        cur.remove(cur.len() - 1);
    }

    result.iter().fold(Vec::new(), |mut agg, vs| {
        for v in vs.iter() {
            agg.push(v.to_vec());
        }
        agg
    })
}

fn get_cube_index(indices: &Vec<usize>, dimensions: &Vec<usize>) -> usize {
    let mut i = 0;
    for (j,index) in indices.iter().enumerate() {
        let mut m = 1; 
        for d in dimensions.iter().skip(j + 1) {
            m *= *d;
        }
        i += *index * m;
    }
    i
}

fn break_cube_index(index: usize, dimensions: &Vec<usize>) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();
    for i in 0..dimensions.len() {
        let width = dimensions.iter().skip(i+1).fold(1usize, |s,d| s * d);
        let base = indices.iter().enumerate()
            .fold(0isize, |s,(j,ind)| s + *ind as isize * dimensions.iter().skip(j+1).fold(1isize, |s,d| s * *d as isize));
        let v = (index as isize - base) as usize
            / width;
        indices.push(v);
    }
    indices
}

fn read_lines(filename: String, cycles: usize, dimension: usize) -> (Vec<Cube>, Vec<usize>) {
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
    
     let extra_dimensions = (dimension as isize - 2) as usize;
     let extra_dimensions_len = (2 * cycles) + 1;
     let mut cubes = vec![Cube::Inactive; (2 * cycles + cube_lines.len()) * cube_lines[0].len() * extra_dimensions_len.pow(extra_dimensions  as u32)];
     let mut dimensions = Vec::new();
     for _ in 0..extra_dimensions {
         dimensions.push(extra_dimensions_len);
     }
     dimensions.push(cube_lines.len() + cycles * 2);
     dimensions.push(cube_lines[0].len());
     for (i, cl) in cube_lines.iter().enumerate() {
        for (j, c) in cl.iter().enumerate() {
            let mut pos = vec![cycles; extra_dimensions];
            pos.push(i+cycles);
            pos.push(j);
            cubes[get_cube_index(&pos, &dimensions)] = *c;
        }
     }

     (cubes, dimensions)
}