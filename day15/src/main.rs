use std::collections::HashMap;

fn main() {
	let input = [19,0,5,1,10,13];

    let num_turns = 2020;
    let turns = play(&input, num_turns);
	println!("{}", turns[num_turns - 1]);

    let num_turns = 30000000;
    let turns = play(&input, num_turns);
    println!("{}", turns[num_turns - 1]);
}

fn play(input: &[i32], num_turns: usize) -> Vec::<i32> { 
	let mut turns = Vec::with_capacity(num_turns);
	let mut memory: HashMap::<i32, Vec<usize>>  = HashMap::new();
	for (i, v) in input.iter().enumerate() {
		commit(*v, i, &mut turns, &mut memory);
	}

	for i in input.len()..num_turns {
		let prev = turns[i-1];		
		let turn_history = memory.get(&prev).unwrap();
		let value = if turn_history.len() == 1 {
            0
		} else {
			(turn_history[turn_history.len()-1] - turn_history[turn_history.len()-2]) as i32
		};
		commit(value, i, &mut turns, &mut memory);
	}

	turns
}

fn commit(value: i32, turn: usize, turns: &mut Vec<i32>, memory: &mut HashMap<i32, Vec<usize>>) {
	turns.push(value);
	if memory.contains_key(&value) {
		memory.get_mut(&value).unwrap().push(turn);
	} else {
		memory.insert(value, vec!(turn;1));
	}
}