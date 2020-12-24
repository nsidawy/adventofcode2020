fn main() {
    //let input = "389125467";
    let input = "123487596";
    let total = input.len();
    let (curr, next) = get_cups(input, total);
    let (_, next) = play(curr, next, 100);
    print(&next);

    let total = 1000000;
    let (curr, next) = get_cups(input, total);
    let (_, next) = play(curr, next, 10000000);
    let v1 = next[0];
    let v2 = next[v1];
    println!("{}", (v1+1)*(v2+1));
}

fn print(next: &Vec<usize>) {
    let mut current = next[0];
    print!("{}",1);
    while current != 0 {
        print!("{}", current+1);
        current = next[current];
    }
    println!("");
}

fn get_cups(input: &str, total: usize) -> (usize, Vec<usize>) {
    let ints: Vec<usize> = input.chars().map(|c| (c.to_digit(10).unwrap()-1) as usize).collect();
    let mut next = vec![0usize; total];
    for i in 0..ints.len()-1 {
        next[ints[i]] = ints[i+1];
    }
    let mut prev = ints[ints.len()-1];
    for i in ints.len()..total {
        next[prev] = i;
        prev = i;
    }
    next[prev] = ints[0];

    (ints[0], next)
}

fn play(mut current_id: usize, mut next: Vec<usize>, moves: u32) -> (usize, Vec<usize>) {
    for _ in 0..moves {
        //1.
        let cup1 = next[current_id];
        let cup2 = next[cup1];
        let cup3 = next[cup2];
        next[current_id] = next[cup3];
    
        //2.
        let mut destination = if current_id == 0 { next.len()-1 } else { current_id - 1 }; 
        while destination == cup1 || destination == cup2 || destination == cup3 {
            destination = if destination == 0 { next.len()-1 } else { destination - 1 }; 
        };

        //3.
        next[cup3] = next[destination];
        next[destination] = cup1;

        current_id = next[current_id];
    }

    (current_id, next)
}
