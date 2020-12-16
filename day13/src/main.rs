use std::env;
use std::fs::File;
use std::io::{self, BufRead};

struct Bus {
    offset: i64,
    id: i64,
}

fn main() {   
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 

    let (time, mut buses) = get_input(path);
    let (bus_id, next_bus_time) = get_next_bus(time, &buses);
    println!("Part 1: {}", bus_id * (next_bus_time - time));
    println!("Part 2: {}", calc_bus_thing(&mut buses));

}

fn get_next_bus(time: i64, buses: &Vec<Bus>) -> (i64, i64) {
    let ids = buses.into_iter().map(|b| b.id);
    ids.map(|id| {
            let prev_time = (time / id) * id;
            let next_time = if prev_time < time {
                prev_time + id
            } else {
                prev_time
            };
            (id, next_time)
        })
        .min_by_key(|(_, nt)| *nt)
        .unwrap()
}

fn calc_bus_thing(buses: &mut Vec<Bus>) -> i64 {
    let first_bus = buses.get(0).unwrap();
    let mut increment = first_bus.id;
    let mut start = first_bus.id - first_bus.offset;
    for bus in buses.iter().skip(1) {
        let first = loop {
            if (start + bus.offset) % bus.id == 0 {
                break start
            }
            start += increment;
        };
        start += increment;
        let second = loop {
            if (start + bus.offset) % bus.id == 0 {
                break start
            }
            start += increment;
        };
        increment = second - first;
        start = first;
    }

    start
}

fn get_input(filename: String) -> (i64,Vec::<Bus>) {
    let file = File::open(filename).unwrap();
    let mut lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    let time = lines.next().unwrap().parse::<i64>().unwrap();
    let buses = lines.next().unwrap()
        .split(",")
        .enumerate()
        .map(|(i, n)| {
            let parse_n = n.parse::<i64>();
            if parse_n.is_ok() {
                Some(Bus { offset: i as i64, id: parse_n.unwrap() } )
            } else {
                None
            }
        })
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
        .collect();

    (time, buses)
}