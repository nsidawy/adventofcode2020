use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

#[derive(Debug, Clone)]
enum Rule {
    Reference(Vec<Vec<u32>>),
    Recurse(Vec<u32>, Vec<u32>),
    Literal(char)
}

impl Rule {
    pub fn parse(rule_id: u32, text: &str) -> Rule {
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

        if references.iter().any(|r| r.contains(&rule_id)) {
            let base = references.iter().find(|r| !r.contains(&rule_id)).unwrap().clone();
            let recurse = references.iter().find(|r| r.contains(&rule_id)).unwrap().clone();
            return Rule::Recurse(base, recurse)
        }
        Rule::Reference(references)
    }
}

fn main() {   
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let (mut rules, lines) = read_lines(path);
    let rule_ids = rules.iter().map(|(k, _)| *k).collect();
    blow_recurse(&rule_ids, &mut rules);
    let valid_count = lines.iter().filter(|l| is_valid(0, l, &rules).0).count();
    println!("Part 1: {:#?}", valid_count);

    let path = format!("{}\\input\\input2.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let (mut rules, lines) = read_lines(path);
    let rule_ids = rules.iter().map(|(k, _)| *k).collect();
    blow_recurse(&rule_ids, &mut rules);
    let valid_count = lines.iter().filter(|l| is_valid(0, l, &rules).0).count();
    println!("Part 2: {:#?}", valid_count);
}

fn blow_recurse(rule_ids: &Vec<u32>, rules: &mut HashMap<u32,Rule>) {
    for rule_id in rule_ids {
        let rule = rules.get(rule_id).unwrap();
        match rule {
            Rule::Reference(ref_list) => {
                if !ref_list.iter().any(|rl| rl.iter()
                    .any(|r| match rules.get(r).unwrap() { Rule::Recurse(_,_) => true, _ => false})){
                        continue;
                    }
                    let ref_list = &ref_list[0];
                    let mut new_list: Vec<Vec<u32>> = Vec::new();
                    for r in ref_list {
                        let rule_dep = rules.get(&r).unwrap();
                        let refs = match rule_dep {
                            Rule::Recurse(base, recurse) => {
                                let mut recurse_refs: Vec<Vec<u32>> = Vec::new();
                                for i in 0..5 {
                                    let mut cur: Vec<u32> = recurse.clone();
                                    for j in 0..i{
                                        let mut next_cur = Vec::new();
                                        for r1 in cur {
                                            if r1 == *r {
                                                for rc in recurse {
                                                    next_cur.push(*rc);
                                                }
                                            } else {
                                                next_cur.push(r1);
                                            }
                                        }
                                        cur = next_cur;
                                    }                                                                       
                                    let index = cur.iter().position(|x| *x == *r).unwrap();
                                    cur.remove(index);
                                    recurse_refs.push(cur);
                                }
                                recurse_refs
                            },
                            _ => vec!(vec!(*r)) 
                        };
                        if new_list.len() == 0 {
                            new_list = refs;
                            continue;
                        }
                        let mut next_new_list: Vec<Vec<u32>> = Vec::new();
                        for rf in refs {
                            for l in new_list.iter() {
                                let mut n = l.clone();
                                n.extend(rf.clone());
                                next_new_list.push(n);
                            }
                        }
                        new_list = next_new_list;
                    }
                    rules.insert(*rule_id, Rule::Reference(new_list));
            },
            _ => ()
        }
    }
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
        },
        _ => (false, String::from(""))
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
        rules.insert(rule_id, Rule::parse(rule_id, &rule_text));
    }

    (rules, lines.map(|l| l.unwrap()).filter(|l| l.len() > 0).collect())
}