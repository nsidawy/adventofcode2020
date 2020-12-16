use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    N,
    W,
    S,
    E,
}

enum TurnDirection {
    Left,
    Right,
}

enum Instruction {
    Move(Direction, i32),
    Forward(i32),
    Turn(TurnDirection, i32),
}

impl Instruction {
    pub fn parse(text: String) -> Option<Instruction> {
        let  mut chars = text.chars();
        let first = chars.next().unwrap();
        let value = chars.collect::<String>().parse::<i32>().unwrap();
        match first {
            'N' => Some(Instruction::Move(Direction::N, value)),
            'E' => Some(Instruction::Move(Direction::E, value)),
            'S' => Some(Instruction::Move(Direction::S, value)),
            'W' => Some(Instruction::Move(Direction::W, value)),
            'F' => Some(Instruction::Forward(value)),
            'R' => Some(Instruction::Turn(TurnDirection::Right, value / 90)),
            'L' => Some(Instruction::Turn(TurnDirection::Left, value / 90)),
            _ => None
        }
    }
}

#[derive(Clone, Copy)]
struct Ship {
    direction: Direction,
    x_pos: i32,
    y_pos: i32,
}

impl Ship {
    pub fn new() -> Ship {
        Ship { direction: Direction::E, x_pos: 0, y_pos: 0 }
    }

    pub fn step(self, instruction: Instruction) -> Ship {
        match instruction{
            Instruction::Move(d, n) => {
                match d {
                    Direction::N => Ship { y_pos: self.y_pos + n, ..self },
                    Direction::S => Ship { y_pos: self.y_pos - n, ..self },
                    Direction::E => Ship { x_pos: self.x_pos + n, ..self },
                    Direction::W => Ship { x_pos: self.x_pos - n, ..self },
                }
            },
            Instruction::Forward(n) => self.step(Instruction::Move(self.direction, n)),
            Instruction::Turn(td, n) => {
                let ordered_directions = [Direction::N, Direction::E, Direction::S, Direction::W];
                let current_index = ordered_directions.iter().position(|d| *d == self.direction).unwrap() as i32;
                let index_delta: i32 = n * match td {TurnDirection::Left => -1, TurnDirection::Right => 1};
                let new_index = (index_delta + current_index + 4) % 4;
                Ship { 
                    direction: ordered_directions[new_index as usize].clone(),
                    ..self
                }
            },
        }
    }
}

fn main() {   
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let instructions = read_lines(path);
    let ship = instructions.into_iter().fold(Ship::new(), |ship, instruction| ship.step(instruction));
    
    println!("Part 1: {} {} = {}", ship.x_pos, ship.y_pos, ship.x_pos.abs() + ship.y_pos.abs());
}
fn read_lines(filename: String) -> Vec<Instruction> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines().map(|l| l.unwrap()).filter(|l| l.len() > 0);
    lines.map(|l| Instruction::parse(l).expect("parse failure")).collect()
}