use std::env;
use std::collections::*;
use std::fs::File;
use std::io::{self, BufRead};
use joinery::Joinable;

fn main() {   
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let (deck1, deck2) = read_lines(path);
    println!("{:#?}", part1(deck1, deck2));
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let (deck1, deck2) = read_lines(path);
    println!("{}", part2(deck1, deck2));
}

fn part1(deck1: VecDeque<u32>, deck2: VecDeque<u32>) -> u32 {
    let (deck1, deck2) = play(deck1, deck2);
    let winner = if deck1.is_empty() { deck2 } else { deck1 };
    winner.iter().rev().enumerate().fold(0u32, |s,(i,d)| s + (i as u32 + 1) * d)
}
fn part2(deck1: VecDeque<u32>, deck2: VecDeque<u32>) -> u32 {
    let (p1_win, deck1, deck2) = play_recursive(deck1, deck2);
    let winner = if p1_win { deck1 } else { deck2 };
    winner.iter().rev().enumerate().fold(0u32, |s,(i,d)| s + (i as u32 + 1) * d)
}

fn play_recursive(mut deck1: VecDeque<u32>, mut deck2: VecDeque<u32>) -> (bool, VecDeque<u32>,VecDeque<u32>) {
    let mut previous = HashSet::new();
    while !deck1.is_empty() && !deck2.is_empty() {
        let card_id = get_cards_id(&deck1, &deck2);
        if previous.contains(&card_id) {
            return (true, deck1, deck2);
        }
        previous.insert(card_id);

        let d1 = deck1.pop_front().unwrap();
        let d2 = deck2.pop_front().unwrap();
        if d1 as usize <= deck1.len() && d2 as usize <= deck2.len() {
            let s_d1 = deck1.iter().take(d1 as usize).map(|d| *d).collect();
            let s_d2 = deck2.iter().take(d2 as usize).map(|d| *d).collect();
            let (p1_win, _, _) = play_recursive(s_d1, s_d2);
            if p1_win {
                deck1.push_back(d1);
                deck1.push_back(d2);
            } else {
                deck2.push_back(d2);
                deck2.push_back(d1);
            }
        } else {
            if d1 > d2 {
                deck1.push_back(d1);
                deck1.push_back(d2);
            } else {
                deck2.push_back(d2);
                deck2.push_back(d1);
            }
        }
    }

    (deck2.is_empty(), deck1, deck2)
}

fn get_cards_id(deck1: &VecDeque<u32>, deck2: &VecDeque<u32>) -> String {
    let mut id: String = deck1.clone().join_with(",").to_string();
    id.push_str("|");
    id.push_str(&deck2.clone().join_with(",").to_string());
    id
}

fn play(mut deck1: VecDeque<u32>, mut deck2: VecDeque<u32>) -> (VecDeque<u32>,VecDeque<u32>) {
    while !deck1.is_empty() && !deck2.is_empty() {
        let d1 = deck1.pop_front().unwrap();
        let d2 = deck2.pop_front().unwrap();
        if d1 > d2 {
            deck1.push_back(d1);
            deck1.push_back(d2);
        } else {
            deck2.push_back(d2);
            deck2.push_back(d1);
        }
    }
    (deck1, deck2)
}

fn read_lines(filename: String) -> (VecDeque<u32>, VecDeque<u32>) {
    let file = File::open(filename).unwrap();
    let mut lines = io::BufReader::new(file).lines();

    lines.next();
    let mut deck1 = VecDeque::new();
    while let Ok(d) = lines.next().unwrap().unwrap().parse::<u32>() {
        deck1.push_back(d);
    }
    lines.next();
    let mut deck2 = VecDeque::new();
    while let Ok(d) = lines.next().unwrap().unwrap().parse::<u32>() {
        deck2.push_back(d);
    }

    (deck1, deck2)
}
