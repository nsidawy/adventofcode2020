use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
use chrono::prelude::*;

#[derive(Debug, Clone, PartialEq, Hash)]
struct Point {
    coords: Vec<i32>,
}

impl Point {
    pub fn get_neighbors(&self) -> Vec<Point>{
        let neighbor_coords:Vec<Vec<i32>> = self.coords.iter()
            .map(|ind| vec!(*ind, *ind + 1, *ind - 1)).collect();        

        let mut curr_n: Vec<Vec<i32>> = Vec::new();
        for nc in neighbor_coords {
            let mut next_n: Vec<Vec<i32>> = Vec::new();
            for n in nc {
                if curr_n.len() == 0 {
                    next_n.push(vec!(n));
                    continue;
                } 
                for c in &curr_n {
                    let mut x = c.clone();
                    x.push(n);
                    next_n.push(x);
                }
            } 
            curr_n = next_n;
        }

        curr_n
             .iter()
             .map(|coords| Point {coords: coords.to_vec()})
             .filter(|p| *p != *self)
             .collect()
    }
}

impl Eq for Point {}

fn main() {   
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let cycles = 6usize;
    let dimension = 4usize;
    let active_cubes = read_lines(path, dimension);
    let active_cubes = step(active_cubes, cycles);
    let active_count = active_cubes.len();

    println!("{:#?}", active_count);
}

fn step(active_cubes: HashSet<Point>, cycles: usize) -> HashSet<Point> {
    println!("cycle {} {:?}", cycles, Local::now().format("%H:%M:%S.%f").to_string());
    if cycles == 0 {
        return active_cubes;
    }
    let mut next_cubes = HashSet::new();
    let neighbors = active_cubes.iter().map(|a| a.get_neighbors());
    for neighbor in neighbors {
        for n in neighbor {
            let neighbors = n.get_neighbors();
            let count = neighbors.iter().fold(0u32, |s,p| s + if active_cubes.contains(p) {1} else {0});
            if active_cubes.contains(&n) {
                if count == 2 || count ==3 {
                    next_cubes.insert(n);
                }
            }  else if count == 3 {
                next_cubes.insert(n);
            }
        }
    }
    for a in active_cubes.iter() {
        let neighbors = a.get_neighbors();
        let count = neighbors.iter().fold(0u32, |s,p| s + if active_cubes.contains(p) {1} else {0});
        if count == 2 || count ==3 {
            next_cubes.insert(a.clone());
        }
    }
    step(next_cubes, cycles - 1)
}

fn read_lines(filename: String, dimension: usize) -> HashSet<Point> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut active_cubes: HashSet<Point> = HashSet::new();
    let extradimensions = (dimension as isize - 2) as usize;
    for (i, line) in lines.enumerate() {
        let line = line.unwrap();
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                let mut coords = vec![0i32; extradimensions]; 
                coords.push(i as i32);
                coords.push(j as i32);
                active_cubes.insert(Point {coords});
            }
        }
    }

    active_cubes
}