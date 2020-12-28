use std::rc::Rc;

mod mylist;

fn main() {
    //let input = "389125467";
    let input = "123487596";
    let total = input.len();
    let (curr, next) = get_cups(input, total);
    let curr = play(curr, &next, 100);
    println!("{}", curr.count());
    curr.print();

    let total = 1000000;
    let (curr, next) = get_cups(input, total);
    println!("{}", curr.count());
    let _ = play(curr, &next, 10000000);
    let start = Rc::clone(&(next[1]));
    println!("{}", start.count());
    let n1 = start.next().unwrap();
    let n2 = n1.next().unwrap();
    let v1 = n1.value().unwrap();
    let v2 = n2.value().unwrap();
    println!("{}", v1*v2);
}

fn get_cups(input: &str, total: usize) -> (Rc<mylist::CupList>, Vec<Rc<mylist::CupList>>) {
    let ints: Vec<usize> = input.chars().map(|c| (c.to_digit(10).unwrap()) as usize).collect();
    let mut next = vec![Rc::new(mylist::CupList::new_nil()); total + 1];
    let mut current = Rc::new(mylist::CupList::new_nil());
    let mut first = None;
    for i in (ints.len()+1..total+1).rev().chain((0..ints.len()).rev()) {
        let value = if i < ints.len() { ints[i] } else { i };
        current = Rc::new(mylist::CupList::new(value, &current));
        next[value] = Rc::clone(&current);
        if first.is_none() {
            first = Some(Rc::clone(&current));
        }
    }
    first.unwrap().set_next(&current);
    (current, next)
}

fn play(mut current: Rc<mylist::CupList>, next: &Vec<Rc<mylist::CupList>>, moves: u32) -> Rc<mylist::CupList> {
    for _ in 0..moves {
        //1.
        let cup1 = current.next().unwrap();
        let cup2 = cup1.next().unwrap();
        let cup3 = cup2.next().unwrap();
        current.set_next(&cup3.next().unwrap());
    
        //2.
        let c = current.value().unwrap();
        let c1 = cup1.value().unwrap();
        let c2 = cup2.value().unwrap();
        let c3 = cup3.value().unwrap();
        let mut destination = if c == 1 { next.len()-1 } else { c - 1 }; 
        while destination == c1 || destination == c2 || destination == c3 {
            destination = if destination == 1 { next.len()-1 } else { destination - 1 }; 
        };

        //3.
        let destination = Rc::clone(&next[destination]);
        cup3.set_next(&destination.next().unwrap());
        destination.set_next(&cup1);

        current = current.next().unwrap();
    }

    current
}
