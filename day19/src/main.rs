use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

#[derive(Debug, Clone)]
enum Rule {
    Reference(Vec<Vec<u32>>),
    Literal(char)
}

impl Rule {
    pub fn parse(text: &str) -> Rule {
        let literal_regex = Regex::new(r#""(\w)""#).expect("broken literal regex");
        if literal_regex.is_match(text) {
            let caps = literal_regex.captures(text).expect("could not capture literal");
            let literal = caps.get(1).unwrap().as_str();
            return Rule::Literal(literal.chars().nth(0).unwrap())
        }

        let words = text.split(" ");
        let mut references = Vec::new();
        let mut cur_reference = Vec::new();
        for word in words {
            match word {
                "|" => {
                    references.push(cur_reference);
                    cur_reference = Vec::new();
                },
                n => cur_reference.push(n.parse::<u32>().expect("can't parse int")),
            }
        }
        references.push(cur_reference);

        Rule::Reference(references)
    }
}

fn main() {   
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let (rules, lines) = read_lines(path);
    let valid_count = lines.iter().filter(|l| is_valid(l, &rules)).count();
    println!("Part 1: {:#?}", valid_count);

    let path = format!("{}\\input\\input2.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let (rules, lines) = read_lines(path);
    let valid_count = lines.iter().filter(|l| is_valid(l, &rules)).count();
    println!("Part 2: {:#?}", valid_count);
}

fn is_valid(text: &String, rules: &HashMap<u32, Rule>) -> bool {
   apply_rules(0, text, &rules).iter().any(|r| r == "")
}

fn apply_rules(rule_id: u32, text: &String, rules: &HashMap<u32, Rule>) -> Vec<String> {
    let rule  = rules.get(&rule_id).unwrap();
    match rule {
        Rule::Literal(c) => {
            let mut chars = text.chars();
            let c1 = chars.nth(0);
            if c1.is_some() && c1.unwrap() == *c {
                return vec!(String::from(chars.as_str()));
            }
            vec!()
        },
        Rule::Reference(references) => {
            let mut results = Vec::new();
            for reference_list in references {
                let mut texts = vec!(text.clone());
                for reference in reference_list {
                     texts = texts.iter().flat_map(|t| apply_rules(*reference, &t, rules)).collect();
                     if texts.len() == 0 {
                         break;
                     }                    
                }
                results.extend(texts);
            }

            results
        },
    }
}

fn read_lines(filename: String) -> (HashMap<u32, Rule>, Vec<String>) {
    let file = File::open(filename).unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut rules = HashMap::new();
    let rule_regex = Regex::new(r"(\d+): (.*)").unwrap();

    loop {
        let line = lines.next().unwrap().unwrap();
        if line.len() == 0 {
            break;
        }
        let caps = rule_regex.captures(&line).unwrap();
        let rule_id = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let rule_text = caps.get(2).unwrap().as_str();
        rules.insert(rule_id, Rule::parse(&rule_text));
    }

    (rules, lines.map(|l| l.unwrap()).filter(|l| l.len() > 0).collect())
}