use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashMap;

type Ticket = Vec<i32>;

struct Range {
    min: i32,
    max: i32,
}

struct Rule {
    name: String,
    ranges: Vec<Range>
}

impl Rule {
    pub fn parse(text: String) -> Rule {
        let name_regex = Regex::new(r"(.+):").unwrap();
        let range_regex = Regex::new(r"(\d+)-(\d+)").unwrap();
        let mut ranges = Vec::new();
        for capture in range_regex.captures_iter(&text) {
            let min = capture.get(1).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap();
            let max = capture.get(2).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap();
            ranges.push(Range { min, max});
        }

        Rule {
            name: name_regex.captures(&text).unwrap().get(1).map(|m| m.as_str()).unwrap().to_string(),
            ranges
        }
    }
}

fn main() {
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let (rules, my_ticket, tickets) = get_input(path);
    let (valid_tickets, invalid_count) = validate_tickets(&rules, tickets);
    let fields = get_fields(&rules, &valid_tickets);

    let result = fields.iter().fold(1i64, |m, (i, name)| {
        if name.split(" ").next().unwrap() == "departure" {
            return m * my_ticket[*i] as i64;
        }
        m
    });
    println!("{} {:#?}", invalid_count, result);
}

fn get_fields(rules: &Vec<Rule>, tickets: &Vec<Ticket>) -> HashMap::<usize, String> {
    let mut possibilities = Vec::new();    
    for rule in rules {
        let mut is_field = vec!(true; tickets[0].len());
        for ticket in tickets {
            for (i, value) in ticket.iter().enumerate() {
                let mut found = false;
                for range in rule.ranges.iter() {
                    if *value >= range.min && *value <= range.max {
                        found = true;
                        break;
                    }
                }
                if !found {
                    is_field[i] = false;
                    break;
                }
            }
        }
        possibilities.push((rule.name.clone(), is_field));
    }

    let mut map = HashMap::new();
    while !possibilities.is_empty() {
        for i in 0..possibilities.len() {
            let possibility = &possibilities[i];
            if possibility.1.iter().enumerate()
                    .filter(|(j,f)| **f && !map.contains_key(j))
                    .count() == 1 {
                let index = possibility.1.iter().enumerate()
                    .position(|(j,b)| *b && !map.contains_key(&j)).unwrap();
                map.insert(index, possibility.0.clone());
                possibilities.remove(i);
                break;
            }
        }
    }

    map
}

fn validate_tickets(rules: &Vec<Rule>, tickets: Vec<Ticket>) -> (Vec::<Ticket>, i32) {
    let mut count = 0;
    let mut valid_tickets = Vec::new();
    for ticket in tickets {
        let mut is_valid = true;
        for value in ticket.iter() {
            let mut found = false;
            for rule in rules {
                for range in rule.ranges.iter() {
                    if *value >= range.min && *value <= range.max{
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                count += value;
                is_valid = false;
            } 
        }
        if is_valid {
            valid_tickets.push(ticket);
        }
    }

    (valid_tickets, count)
}

fn get_input(filename: String) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let file = File::open(filename).unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut rules = Vec::new();
    loop {
        let l = lines.next().unwrap().unwrap();
        if l.len() == 0 {
            break;
        }
        rules.push(Rule::parse(l));
    }

    lines.next();
    let my_ticket = lines.next().unwrap().unwrap()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    lines.next();
    lines.next();
    let mut tickets = Vec::new();
    for line in lines {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }

        tickets.push(line.split(",").map(|s| s.parse::<i32>().unwrap()).collect());
    }
    

    (rules, my_ticket, tickets)
}