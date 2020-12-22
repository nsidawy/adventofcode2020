use std::env;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use itertools::Itertools;
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
    let (mut allergen_possibilities, allergen_free_incredients) = get_allergen_free_ingredients(&foods);
    let appearence_count = foods.iter().fold(0u32, |s,f| {
        let mut count = 0;
        for afi in allergen_free_incredients.iter() {
            if f.ingredients.contains(afi) {
                count += 1;
            }
        }
        s + count
    });

    let mut allergen_matches = get_allergen_matches(&mut allergen_possibilities);
    allergen_matches.sort_by_key(|(a,_)| String::from(a));
    let list = allergen_matches.into_iter().map(|(_,i)| i).collect::<Vec<String>>().join_with(",").to_string();

    println!("{:#?}", allergen_free_incredients);
    println!("{:#?}", appearence_count);
    println!("{:#?}", list);
}

fn get_allergen_matches(allergen_possibilities: &mut HashMap<String,HashSet<String>>) -> Vec<(String,String)> {
    let mut matches = Vec::new();

    while !allergen_possibilities.is_empty() {
        let mut new_matches = vec!();
        for (a, is) in allergen_possibilities.iter() {
            if is.len() == 1 {
                new_matches.push((String::from(a), String::from(is.iter().next().unwrap())));
            }
        }

        for (a,i) in new_matches.iter() {
            allergen_possibilities.remove(a);
            for ap_a in allergen_possibilities.values_mut() {
                ap_a.remove(i);
            }
        }

        matches.extend(new_matches);
    }

    matches
}

fn get_allergen_free_ingredients(foods: &Vec<Food>) -> (HashMap<String,HashSet<String>>, Vec<String>) {
    let mut allergen_possibilities: HashMap<String,HashSet<String>> = HashMap::new();

    for food in foods {
        for allergen in food.allergens.iter() {
            if allergen_possibilities.contains_key(allergen) {
                let mut ap = allergen_possibilities.get(allergen).unwrap().clone();
                for a in allergen_possibilities.get(allergen).unwrap().iter() {
                    if !food.ingredients.contains(a) {
                        ap.remove(a);
                    }
                }
                allergen_possibilities.insert(allergen.to_string(), ap);
            } else {
                allergen_possibilities.insert(allergen.to_string(), food.ingredients.clone());
            }
        }
    }

    let ingredients = foods.iter().flat_map(|f| f.ingredients.clone()).collect::<Vec<String>>().into_iter().unique();
    let mut allergen_free_ingredients = vec!();
    for ingredient in ingredients {
        let mut found = false;
        for ap in allergen_possibilities.values() {
            if ap.contains(&ingredient) {
                found = true;
                break;
            }            
        } 
        if !found {
            allergen_free_ingredients.push(ingredient);
        }
    }
    println!("{:#?}", allergen_possibilities);
    (allergen_possibilities, allergen_free_ingredients)
}

fn read_lines(filename: String) -> Vec<Food> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.filter_map(|l| Food::build(l.unwrap())).collect()
}
