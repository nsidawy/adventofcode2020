use std::env;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use joinery::Joinable;

#[derive(Debug)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>
}

impl Food {
    pub fn build(line: String) -> Option<Food> {
        let regex = Regex::new(r"(.*) \(contains (.*)\)").unwrap();
        let capture = regex.captures(&line);
        if capture.is_none() {
            return None;
        }

        let capture = capture.unwrap();
        let ingredients_text = capture.get(1).unwrap().as_str();
        let ingredients = ingredients_text.split(" ").map(|s| String::from(s)).collect();

        let alergens_text = capture.get(2).unwrap().as_str();
        let allergens = alergens_text.split(", ").map(|s| String::from(s)).collect();

        Some(Food { ingredients, allergens })
    }
}

fn main() {   
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let foods = read_lines(path);
    let mut allergen_possibilities = get_allergen_possibilities(&foods);
    let allergens: HashSet<String> = allergen_possibilities.iter().flat_map(|(_,is)| is.iter().map(|i| String::from(i))).collect();
    let appearence_count = foods.into_iter().flat_map(|f| f.ingredients).filter(|i| !allergens.contains(i)).count();
    let mut allergen_matches = get_allergen_matches(&mut allergen_possibilities);
    allergen_matches.sort_by_key(|(a,_)| String::from(a));
    let list = allergen_matches.into_iter().map(|(_,i)| i).collect::<Vec<String>>().join_with(",").to_string();

    println!("{:#?}", appearence_count);
    println!("{:#?}", list);
}

fn get_allergen_matches(allergen_possibilities: &mut HashMap<String,HashSet<String>>) -> Vec<(String,String)> {
    let mut matches = Vec::new();

    while let Some((a,_)) = allergen_possibilities.iter().find(|(_,is)| is.len() == 1) {
        let i = String::from(allergen_possibilities.get(a).unwrap().iter().next().unwrap());
        let a = String::from(a);
        for ap_a in allergen_possibilities.values_mut() {
            ap_a.remove(&i);
        }
        matches.push((a, i));
    }
    matches
}

fn get_allergen_possibilities(foods: &Vec<Food>) -> HashMap<String,HashSet<String>> {
    let mut allergen_possibilities: HashMap<String,HashSet<String>> = HashMap::new();
    for food in foods {
        for allergen in food.allergens.iter() {
            if allergen_possibilities.contains_key(allergen) {
                let ap = allergen_possibilities.get_mut(allergen).unwrap();
                *ap = ap.intersection(&food.ingredients).map(|i| String::from(i)).collect::<HashSet<_>>();
            } else {
                allergen_possibilities.insert(allergen.to_string(), food.ingredients.clone());
            }
        }
    }
    allergen_possibilities
}

fn read_lines(filename: String) -> Vec<Food> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.filter_map(|l| Food::build(l.unwrap())).collect()
}
