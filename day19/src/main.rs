use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

#[derive(Debug)]
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
    let path = format!("{}\\input\\input2.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let (mut rules, lines) = read_lines(path);

    let mut res31 = generate(31, &rules);
    let mut res42 = generate(42, &rules);
    res31.sort();
    res42.sort();

    let mut reference_list0: Vec<Vec<u32>> = Vec::new();
    let max_len = 15;
    for l in 2..max_len {
        for i8 in 0..l-1 {
            let mut ref_list = vec![42;i8 + 1];
            for i11 in (i8+1)..l {
                ref_list.push(42);
            }
            for i11 in (i8+1)..l {
                ref_list.push(31);
            }
            reference_list0.push(ref_list);
        }
    }
    println!("ref 0 {:?}", reference_list0);
    rules.insert(0, Rule::Reference(reference_list0));

    let valid_count = lines.iter().filter(|l| is_valid(0, l, &rules).0).count();

    println!("{:#?}", valid_count);
}

fn is_valid(rule_id: u32, text: &String, rules: &HashMap<u32, Rule>) -> (bool, String) {
    let rule  = rules.get(&rule_id).unwrap();
    match rule {
        Rule::Literal(c) => {
            let mut chars = text.chars();
            let c1 = chars.nth(0);
            (c1.is_some() && c1.unwrap() == *c, String::from(chars.as_str())) 
        },
        Rule::Reference(references) => {
            for reference_list in references {
                let mut text1 = text.clone();
                let mut success = true;
                for reference in reference_list {
                    let result = is_valid(*reference, &text1, rules);
                    if !result.0 {
                        success = false;
                        break;
                    }
                    text1 = result.1;
                }
                if success {
                    if (rule_id == 0 && text1.len() == 0)
                        || rule_id != 0 {
                            return (true, String::from(text1.as_str()));
                        }
                }
            }

            (false, String::from(""))
        }
    }
}

fn generate(rule_id: u32, rules: &HashMap<u32, Rule>) -> Vec<String>{
    let rule  = rules.get(&rule_id).unwrap();
    match rule {
        Rule::Literal(c) => vec!(String::from(*c)),
        Rule::Reference(references) => {
            let mut results: Vec<String> = Vec::new();
            for reference_list in references {
                let mut reference_results: Vec<String> = Vec::new();
                for reference in reference_list {
                    let dep_results = generate(*reference, rules);
                    if reference_results.len() == 0 {
                        reference_results = dep_results;
                        continue;
                    }
                    let mut next_results: Vec<String> = Vec::new();
                    for rr in reference_results.iter() {
                        for dr in dep_results.iter() {
                            let mut rr = rr.clone();
                            rr.push_str(dr.as_str());
                            next_results.push(rr);
                        }
                    }
                    reference_results = next_results;
                }
                for rr in reference_results {
                    results.push(rr);
                }
            }

            results 
        }
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
        let rule_number = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let rule_text = caps.get(2).unwrap().as_str();
        rules.insert(rule_number, Rule::parse(&rule_text));
    }

    (rules, lines.map(|l| l.unwrap()).filter(|l| l.len() > 0).collect())
}