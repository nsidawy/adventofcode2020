use std::str::Chars;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone,Debug)]
enum OperandType {
    Addition,
    Multiplication
}

impl OperandType {
    pub fn parse(c: char) -> Option<OperandType> {
        match c {
            '+' => Some(OperandType::Addition),
            '*' => Some(OperandType::Multiplication),
            _ => None
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
                '+' | '*' => operands.push(OperandType::parse(c).expect("get parse operand")),
                '(' => nodes.push(Node::parse(chars)),
                ')' => break,
                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => nodes.push(Node::Constant(c.to_digit(10).unwrap() as i64)),
                _ => ()
            }
        }
        Node::Expression(nodes, operands)
    }

    pub fn evaluate(&self) -> i64 {
        match self {
            Node::Constant(n) => *n,
            Node::Expression(nodes, operands) => {
                let mut base = nodes[0].evaluate();
                for i in 1..nodes.len() {
                    let value = nodes[i].evaluate();
                    base = match operands[i-1] {
                        OperandType::Multiplication => base * value,
                        OperandType::Addition => base + value,
                    } 
                }
                base 
            }
        }
    }

    pub fn evaluate2(&self) -> i64 {
        match self {
            Node::Constant(n) => *n,
            Node::Expression(nodes, operands) => {
                let mut base: Option<i64> = None;
                let mut nodes2: Vec<Node> = Vec::new();
                let mut operands2: Vec<OperandType> = Vec::new();
                for i in 0..nodes.len() {
                    let value = nodes[i].evaluate2();
                    if base.is_none() {
                        base = Some(value);
                        continue;
                    }
                    match operands[i-1] {
                        OperandType::Addition => base = Some(base.unwrap() + value),
                        OperandType::Multiplication => {
                            operands2.push(OperandType::Multiplication);
                            nodes2.push(Node::Constant(base.unwrap()));
                            base = Some(value);
                        },
                    } 
                }
                nodes2.push(Node::Constant(base.unwrap()));

                let mut base: Option<i64> = None;
                for i in 0..nodes2.len() {
                    let value = nodes2[i].evaluate2();
                    if base.is_none() {
                        base = Some(value);
                        continue;
                    }
                    base = match operands2[i-1] {
                        OperandType::Addition => Some(base.unwrap() + value),
                        OperandType::Multiplication => Some(base.unwrap() * value),
                    };
                }
                base.unwrap()
            }
        }
    }
}

fn main() {   
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let expressions = read_lines(path);

    let result1 = expressions.iter()
        .map(|e| e.evaluate())
        .sum::<i64>();
    println!("Part 1: {}", result1);
    let result2 = expressions.iter()
        .map(|e| e.evaluate2())
        .sum::<i64>();
    println!("Part 2: {}", result2);
}

fn read_lines(filename: String) -> Vec<Node> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
        .map(|l| l.unwrap().replace(" ", ""))
        .map(|l| Node::parse(&mut l.chars()))
        .collect()
}