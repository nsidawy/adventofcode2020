const DIVISOR: u64 = 20201227; 

fn main() {
    //let door_key = 17807724;
    //let card_key = 5764801;
    let door_key = 6552760;
    let card_key = 13233401;

    let subject_number = 7;
    let door_loop_size = get_loop_size(door_key, subject_number);
    let card_loop_size = get_loop_size(card_key, subject_number);
    println!("{} {}", door_loop_size, card_loop_size);
    println!("{} {}", transform(door_key, card_loop_size), transform(card_key, door_loop_size));
}

fn get_loop_size(pub_key: u64, subject_number: u64) -> usize {
    let mut loop_size = 0;
    let mut value = 1; 
    while value != pub_key {
        value = (value * subject_number) % DIVISOR;
        loop_size += 1;
    }
    loop_size
}

fn transform(subject_number: u64, loop_size: usize) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value = (value * subject_number) % DIVISOR;
    }
    value
}