use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
struct Rule {
    name: String,
    contents: Vec<Content>,
}

#[derive(Debug)]
struct Content {
    name: String,
    count: u32,
}

fn main() {
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let lines = read_lines(path);
    let bag_contents = lines.iter().map(|l| parse(l));
    let mut content_map = HashMap::new();
    for bag_content in bag_contents {
        content_map.insert(bag_content.name, bag_content.contents);
    }

    println!("Part 1: {}", count_containing_bags(&content_map, "shinygold"));
    println!("Part 2: {}", count_total_bags(&content_map, "shinygold"));
}

fn count_total_bags(contents_map: &HashMap<String, Vec<Content>>, bag: &str) -> u32 {
    let mut count = 0u32;
    for content in &contents_map[bag] {
        count += content.count + content.count * count_total_bags(contents_map, &content.name);
    }
    count 
}

fn count_containing_bags(contents_map: &HashMap<String, Vec<Content>>, bag: &str) -> u32{
    let mut count = 0;
    for (name, _) in contents_map {
       if bag_contains(&contents_map, bag, name) {
           count += 1
       } 
    }
    count
}

fn bag_contains(contents_map: &HashMap<String, Vec<Content>>, target_bag: &str, current_bag: &str) -> bool {
    for bag_content in &contents_map[current_bag] {
        if bag_content.name == target_bag || bag_contains(contents_map, target_bag, &bag_content.name) {
            return true;
        }

    }
    false
}

fn parse(text: &str) -> Rule {
    let text = String::from(text)
        .replace(",", "")
        .replace(".", "")
        .replace("bags", "bag");
    let mut words = text.split(" ");
    let name = eat_bag_name(&mut words, "bag");

    // skip contains
    words.next();

    let mut contents: Vec<Content> = Vec::new();
    loop {
        // skip number
        match words.next(){
            None => break,
            Some("no") => {eat_bag_name(&mut words, "bag"); ()},
            Some(num) =>  {
                let contents_name = eat_bag_name(&mut words, "bag");
                contents.push(Content { 
                    name: contents_name,
                    count: num.parse::<u32>().unwrap_or(0)}
                );
            }
        }
    }

    Rule { name, contents }
}

fn eat_bag_name(words: &mut std::str::Split<&str>, end: &str) -> String {
    let mut bag_name = String::from("");
    loop {
        let word = words.next().unwrap();
        if word == end {
            break bag_name;
        }
        bag_name += word;
    }
}

fn read_lines(filename: String) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.map(|line| line.unwrap()).collect()
}