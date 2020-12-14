use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Debug)]
enum Instruction {
    Nop(i32),
    Jmp(i32),
    Acc(i32)
}

#[derive(Debug)]
struct InstructionState {
    acc: i32,
    index: usize
}

#[derive(Debug)]
enum InstructionResult {
    Infinite(i32),
    Terminated(i32)
}

impl Instruction {
    fn build(instruction: &str) -> Option<Instruction> {
        let val = Instruction::parse_int(&instruction[4..]);
        match &instruction[..3]{
            "jmp" => Some(Instruction::Jmp(val)),
            "acc" => Some(Instruction::Acc(val)),
            "nop" => Some(Instruction::Nop(val)),
            _ => None
        }
    }

    fn process(&self, instruction_state: InstructionState) -> InstructionState{
        match &self {
            Instruction::Nop(_) => InstructionState::build(instruction_state.acc, instruction_state.index + 1),
            Instruction::Acc(n) => InstructionState::build(instruction_state.acc + n, instruction_state.index + 1),
            Instruction::Jmp(n) => InstructionState::build(instruction_state.acc, (instruction_state.index as i32 + n) as usize),
        }
    }

    fn parse_int(text: &str) -> i32 {
        let sign = if text.chars().next().unwrap_or_default() == '-' {-1} else {1};
        sign * &text[1..].parse::<i32>().unwrap_or_default()
    }
}

impl InstructionState {
    fn build(acc: i32, index: usize) -> InstructionState {
        InstructionState { acc, index }
    }
}

fn main() {
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let lines = read_lines(path);
    let mut instructions = lines.iter().map(|l| Instruction::build(l).expect(l)).collect();

    let result1 = match execute(&instructions) {
        InstructionResult::Infinite(n) => n,
        InstructionResult::Terminated(n) => n,
    };
    println!("Part 1: {}", result1);

    let mut result2 = 0;
    for i in 0..instructions.len() {
        let original_instruction = instructions[i].clone();
        match reverse_instruction(&original_instruction) {
            Some(n) => instructions[i] = n,
            None => continue
        }
        match execute(&instructions){
            InstructionResult::Infinite(_) => (),
            InstructionResult::Terminated(n) => {result2 = n; break} 
        };
        instructions[i] = original_instruction;
    }
    println!("Part 2: {}", result2);
}

fn reverse_instruction(instruction: &Instruction) -> Option<Instruction> {
    match instruction {
        Instruction::Nop(n) => Some(Instruction::Jmp(*n)),
        Instruction::Jmp(n) => Some(Instruction::Nop(*n)),
        _ => None 
    }
}

fn execute(instructions: &Vec<Instruction>) -> InstructionResult {
    let mut state = InstructionState::build(0, 0);
    let mut visited = vec![false; instructions.len()];

    loop {
        if state.index == instructions.len() {
            break InstructionResult::Terminated(state.acc)
        }
        if visited[state.index] {
            break InstructionResult::Infinite(state.acc)
        }
        visited[state.index] = true;
        let instruction = &instructions[state.index];
        state = instruction.process(state);
    }
}

fn read_lines(filename: String) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.map(|line| line.unwrap()).collect()
}