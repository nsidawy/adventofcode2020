use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashMap;

const BITS: usize = 36;

struct Mask {
    one_mask: i64,
    zero_mask: i64,
    fluctuate: Vec<usize>
}

impl Mask {
    pub fn apply(&self, value: i64) -> i64{
        (value | self.one_mask) & self.zero_mask
    }

    pub fn fluctuate(&self, value: i64) -> Vec<i64> {
        let value = value | self.one_mask;
        let mut values = Vec::new();
        for f in self.fluctuate.iter() {
            if values.len() == 0 {
                values.push(value | Mask::get_one_mask(*f));
                values.push(value & Mask::get_zero_mask(*f));
            }
            else {
                let mut new_vals = Vec::new();
                for v in values.iter() {
                    new_vals.push(v | Mask::get_one_mask(*f));
                    new_vals.push(v & Mask::get_zero_mask(*f));
                } 
                values = new_vals
            }
        }
        values
    }

    pub fn get_one_mask(i: usize) -> i64 {
        1 << (BITS - (i+1))
    }

    pub fn get_zero_mask(i: usize) -> i64 {
        !(1 << (BITS - (i+1)))
    }
}

enum Instruction {
    SetMask(Mask),
    SetMemory(i64,i64)
}

impl Instruction {
    pub fn parse(line: String) -> Option<Instruction> {
        let mem_regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
        let mask_regex = Regex::new(r"mask = (.+)").unwrap();
        if mem_regex.is_match(&line) {
            let capture = mem_regex.captures(&line).unwrap();
            return Some(Instruction::SetMemory(
                capture.get(1).map(|m| m.as_str().parse::<i64>().unwrap()).unwrap(),
                capture.get(2).map(|m| m.as_str().parse::<i64>().unwrap()).unwrap()))
        }
        if mask_regex.is_match(&line) {
            let capture = mask_regex.captures(&line).unwrap();
            let mask_str = capture.get(1).map(|m| m.as_str()).unwrap();
            let mut one_mask = 0 as i64;
            let mut zero_mask = !(0 as i64);
            let mut fluctuate = Vec::new();
            for (i, c) in mask_str.chars().enumerate() {
               match c {
                   '1' => one_mask = one_mask | Mask::get_one_mask(i), 
                   '0' => zero_mask = zero_mask & Mask::get_zero_mask(i),
                   'X' => fluctuate.push(i),
                   _ => ()
               } 
            }
            return Some(Instruction::SetMask(Mask { one_mask, zero_mask, fluctuate }))
        }

        None
    }
}

fn main() {   
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let instructions = get_input(path);
    let memory = process(instructions, false);
    println!("{}", memory.values().sum::<i64>());

    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let instructions = get_input(path);
    let memory = process(instructions, true);
    println!("{}", memory.values().sum::<i64>());
}

fn process(instructions: Vec::<Instruction>, fluctaute: bool) -> HashMap<i64, i64> {
    let mut memory = HashMap::new();
    let mut mask = Mask { zero_mask: !0, one_mask: 0, fluctuate: Vec::new() };
    for instruction in instructions {
        match instruction {
            Instruction::SetMask(m) => mask = m, 
            Instruction::SetMemory(i, v) => {
                if fluctaute {
                    let addressses = mask.fluctuate(i);
                    for address in addressses {
                        memory.insert(address, v);
                    }
                }
                else {
                    memory.insert(i, mask.apply(v));
                }
            }, 
        }
    }
    memory
}

fn get_input(filename: String) -> Vec::<Instruction> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
        .map(|l| Instruction::parse(l.unwrap()))
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
        .collect()
}