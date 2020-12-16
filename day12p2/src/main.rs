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
struct Position(i32,i32);

#[derive(Clone, Copy)]
struct Ship {
    waypoint: Position,
    position: Position,
}

impl Ship {
    pub fn new() -> Ship {
        Ship { position: Position(0, 0), waypoint: Position(10, 1)}
    }

    pub fn step(self, instruction: Instruction) -> Ship {
        let waypoint = self.waypoint;
        match instruction{
            Instruction::Move(d, n) => {
                match d {
                    Direction::N => Ship { waypoint: Position(waypoint.0, waypoint.1 + n), ..self },
                    Direction::S => Ship { waypoint: Position(waypoint.0, waypoint.1 - n), ..self },
                    Direction::E => Ship { waypoint: Position(waypoint.0 + n, waypoint.1), ..self },
                    Direction::W => Ship { waypoint: Position(waypoint.0 - n, waypoint.1), ..self },
                }
            },
            Instruction::Forward(n) => Ship { 
                position: Position(waypoint.0*n + self.position.0, waypoint.1*n + self.position.1),
                ..self
            },
            Instruction::Turn(td, n) => {
                let multiple = match td { TurnDirection::Left => -1, TurnDirection::Right => 1 };
                let mut new_waypoint = self.waypoint;
                for _ in 0..n {
                    new_waypoint = Position(new_waypoint.1 * multiple, new_waypoint.0 * multiple * -1);
                }
                Ship { 
                    waypoint: new_waypoint,
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
    
    println!("Part 1: {} {} = {}", ship.position.0, ship.position.1, ship.position.0.abs() + ship.position.1.abs());
}

fn read_lines(filename: String) -> Vec<Instruction> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines().map(|l| l.unwrap()).filter(|l| l.len() > 0);
    lines.map(|l| Instruction::parse(l).expect("parse failure")).collect()
}