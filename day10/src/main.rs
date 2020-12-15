use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {   
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let mut ints = read_lines(path);
    ints.push(0);
    ints.sort();
    ints.push(ints[ints.len()-1] + 3);

    let (ones, twos, threes) = get_voltage_differnces(&ints);
    println!("Part 1: {} {} {}; answer: {}", ones, twos, threes, ones * threes);

    let mut memoize = vec![0;ints.len()];
    memoize[0] = 1;
    println!("Part 2: {}", count_adapter_combos(&ints[..], &mut memoize));
}

fn count_adapter_combos(sorted_jolts: &[u32], memoize: &mut Vec<u64>) -> u64 {
    if memoize[sorted_jolts.len()-1] != 0 {
        return memoize[sorted_jolts.len() - 1]
    }

    let mut index = 1;
    let mut count = 0;
    loop {
        if index >= sorted_jolts.len() || sorted_jolts[index] - sorted_jolts[0] > 3 {
            memoize[sorted_jolts.len() - 1] = count;
            break count
        }
        count += count_adapter_combos(&sorted_jolts[index..], memoize);
        index += 1;
    }
}

fn get_voltage_differnces(sorted_jolts: &Vec<u32>) -> (u16, u16, u16){
    let mut ones = 0;
    let mut twos = 0;
    let mut threes = 0;
    for i in 1..sorted_jolts.len() {
        match sorted_jolts[i] - sorted_jolts[i-1] {
            1 => ones += 1,
            2 => twos += 1,
            3 => threes += 1,
            _ => ()
        }
    }
    (ones, twos, threes)
}

fn read_lines(filename: String) -> Vec<u32> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines
        .map(|line| line.unwrap().parse::<u32>())
        .filter(|d| d.is_ok())
        .map(|d| d.unwrap())
        .collect()
}