use std::str::Chars;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(Clone,Debug,PartialEq, Hash)]
enum OperandType {
    Addition,
    Multiplication
}

impl Eq for OperandType {}

impl OperandType {
    pub fn parse(c: char) -> Option<OperandType> {
        match c {
            '+' => Some(OperandType::Addition),
            '*' => Some(OperandType::Multiplication),
            _ => None
        }
    }

    pub fn calculate(&self, v1: i64, v2: i64) -> i64 {
        match self {
            OperandType::Multiplication => v1 * v2,
            OperandType::Addition => v1 + v2,
        }
    }
}

#[derive(Clone,Debug)]
enum Node {
    Expression(Vec<Node>, Vec<OperandType>),
    Constant(i64),
}

impl Node {
    pub fn parse(chars: &mut Chars) -> Node {
        let mut nodes: Vec<Node> = Vec::new();
        let mut operands: Vec<OperandType> = Vec::new();
        loop {
            let c = chars.next();
            if c.is_none() {
                break;
            }

            let c = c.unwrap();
            match c {
                ' ' => ,
                '+' | '*' => operands.push(OperandType::parse(c).expect("get parse operand")),
                '(' => nodes.push(Node::parse(chars)),
                ')' => break,
                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => nodes.push(Node::Constant(c.to_digit(10).unwrap() as i64)),
                _ => ()
            }
        }
        Node::Expression(nodes, operands)
    }

    pub fn evaluate(&self, precedence: &Vec<HashSet<OperandType>>) -> i64 {
        match self {
            Node::Constant(n) => *n,
            Node::Expression(nodes, operands) => {
                let mut nodes = nodes.clone();
                let mut operands = operands.clone();
                for operators in precedence {
                    let mut base: Option<i64> = None;
                    let mut nodes2: Vec<Node> = Vec::new();
                    let mut operands2: Vec<OperandType> = Vec::new();
                    for i in 0..nodes.len() {
                        let value = nodes[i].evaluate(precedence);
                        if base.is_none() {
                            base = Some(value);
                            continue;
                        }
                        if operators.contains(&operands[i-1]) {
                            base = Some(operands[i-1].calculate(base.unwrap(), value));
                        } else {
                            operands2.push(operands[i-1].clone());
                            nodes2.push(Node::Constant(base.unwrap()));
                            base = Some(value);
                        }
                    }
                    nodes2.push(Node::Constant(base.unwrap()));
                    nodes = nodes2;
                    operands = operands2;
                }
                nodes[0].evaluate(precedence)
            }
        }
    }
}

fn main() {   
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let expressions = read_lines(path);

    let precedence1 = vec!(
        [OperandType::Addition, OperandType::Multiplication].iter().cloned().collect()
    );
    let result1 = expressions.iter()
        .map(|e| e.evaluate(&precedence1))
        .sum::<i64>();
    println!("Part 1: {}", result1);
    
    let precedence2 = vec!(
        [OperandType::Addition].iter().cloned().collect(),
        [OperandType::Multiplication].iter().cloned().collect(),
    );
    let result2 = expressions.iter()
        .map(|e| e.evaluate(&precedence2))
        .sum::<i64>();
    println!("Part 2: {}", result2);
}

fn read_lines(filename: String) -> Vec<Node> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
        .map(|l| Node::parse(&mut l.unwrap().chars()))
        .collect()
}