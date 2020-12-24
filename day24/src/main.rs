use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Hash,PartialEq,Eq,Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn w(&self) -> Self { Pos { x: self.x-2, y: self.y } }
    pub fn e(&self) -> Self { Pos { x: self.x+2, y: self.y } }
    pub fn sw(&self) -> Self { Pos { x: self.x-1, y: self.y-1 } }
    pub fn se(&self) -> Self { Pos { x: self.x+1, y: self.y-1 } }
    pub fn ne(&self) -> Self { Pos { x: self.x+1, y: self.y+1 } }
    pub fn nw(&self) -> Self { Pos { x: self.x-1, y: self.y+1 } }
    pub fn get_adjacent(&self) -> Vec<Pos> {
        vec!(self.nw(), self.ne(), self.e(), self.w(), self.se(), self.sw())
    }

    pub fn parse_line(line: String) -> Pos {
        let mut pos = Pos { x:0, y:0 };
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            pos = match c {
                'w' => pos.w(),
                'e' => pos.e(),
                's' => match chars.next().expect("eol") {
                    'w' => pos.sw(),
                    'e' => pos.se(),
                    _ => pos
                },
                'n' => match chars.next().expect("eol") {
                    'w' => pos.nw(),
                    'e' => pos.ne(),
                    _ => pos
                },
                _ => pos
            }
        }
        pos
    }
}

fn main() {   
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let lines = read_lines(path);
    let flips = lines.into_iter().map(|l| Pos::parse_line(l)).collect::<Vec<Pos>>();
    let mut black_tiles = flip_tiles(HashSet::new(), flips);
    println!("Part1: {}", black_tiles.len());
    for _ in 0..100 {
        black_tiles = step(black_tiles);
    }
    println!("Part 2: {}", black_tiles.len());
}

fn step(black_tiles: HashSet<Pos>) -> HashSet<Pos> {
    let candidates: HashSet<Pos> = black_tiles.iter()
        .flat_map(|bt| vec!(bt.clone(), bt.ne(), bt.nw(), bt.w(), bt.e(), bt.se(), bt.sw()))
        .collect();
    let flips = candidates.into_iter().filter(|c|{
        let count = c.get_adjacent().iter().fold(0u32, |s,p| s + if black_tiles.contains(p) { 1 } else { 0 });
        let is_black = black_tiles.contains(&c);
        (is_black && (count == 0 || count > 2)) || (!is_black && count == 2)    
    }).collect();
    flip_tiles(black_tiles, flips)
}

fn flip_tiles(mut black_tiles: HashSet<Pos>, flips: Vec<Pos>) -> HashSet<Pos> {
    for flip in flips {
        if black_tiles.contains(&flip) {
            black_tiles.remove(&flip);
        } else {
            black_tiles.insert(flip);
        }
    }
    black_tiles
}

fn read_lines(filename: String) -> Vec<String> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|l| l.unwrap()).collect::<Vec<String>>()
}