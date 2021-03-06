use std::env;
use std::fs::File;
use std::io::{self, BufRead};
mod validations;

struct PassField {
    field: String,
    value: String,
}

struct Rule<'a> {
    field: &'a str,
    string_fn: &'a dyn Fn(&String) -> bool,
}

fn main() {
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let lines = read_lines(path);
    let rules: [Rule; 7] = [
        Rule { field: "byr", string_fn: &validations::validate_byr },
        Rule { field: "iyr", string_fn: &validations::validate_iyr },
        Rule { field: "eyr", string_fn: &validations::validate_eyr },
        Rule { field: "hgt", string_fn: &validations::validate_hgt },
        Rule { field: "hcl", string_fn: &validations::validate_hcl },
        Rule { field: "ecl", string_fn: &validations::validate_ecl },
        Rule { field: "pid", string_fn: &validations::validate_pid }];

    println!("Part 1: {}", get_batch_validation_count(&lines, &rules, false));
    println!("Part 2: {}", get_batch_validation_count(&lines, &rules, true));
}

fn get_batch_validation_count(
    lines: &Vec<String>,
    rules: &[Rule],
    is_strict: bool
) -> i32{
    let mut count = 0;
    let mut current_pass = Vec::new();
    for line in lines {
        if line.len() == 0 {
            if validate_pass(&current_pass, rules, is_strict) {
                count = count + 1;
            }
            current_pass.clear();
        }
        else {
            current_pass.push(line);
        }
    }
    if validate_pass(&current_pass, rules, is_strict) {
        count = count + 1;
    }

    count
}

fn validate_pass(pass: &Vec<&String>, rules: &[Rule], is_strict: bool) -> bool {
    let pass_rules: Vec<PassField> = pass.iter().map(|p| p.split(" ").map(|s| {
            let vals: Vec<&str> = s.split(":").collect(); 
            PassField { field: vals[0].to_string(), value: vals[1].to_string() }}).collect())
        .fold(Vec::<PassField>::new(), |mut all, v: Vec::<PassField>| {all.extend(v); all});
    let mut count = 0;
    for rule in rules {
        for pass_field in &pass_rules {
            if pass_field.field == rule.field.to_string()
                && (!is_strict || (rule.string_fn)(&pass_field.value)) {
                count = count + 1;
                break;
            }
        }
    }

    count == rules.len()
}

fn read_lines(filename: String) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.map(|line| line.unwrap()).collect()
}