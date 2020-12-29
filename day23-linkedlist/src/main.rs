use std::rc::Rc;
mod mylist;

type CupList = mylist::CupList;

fn main() {
    //let input = "389125467";
    let input = "123487596";
    let total = input.len();
    let (curr, cup_pointers) = get_cups(input, total);
    curr.print();
    let curr = play(curr, &cup_pointers, 100);
    curr.print();

    let total = 1000000;
    let (curr, cup_pointers) = get_cups(input, total);
    let curr = play(curr, &cup_pointers, 10000000);
    println!("{}", curr.count());
    let n1 = cup_pointers[1].next().unwrap();
    let n2 = n1.next().unwrap();
    let v1 = n1.value().unwrap();
    let v2 = n2.value().unwrap();
    println!("{}", v1*v2);
}

fn get_cups(input: &str, total: usize) -> (Rc<CupList>, Vec<Rc<CupList>>) {
    let ints: Vec<usize> = input.chars().map(|c| (c.to_digit(10).unwrap()) as usize).collect();
    let mut cup_pointers = vec![Rc::new(CupList::new_nil()); total + 1];
    let mut current = Rc::new(CupList::new_nil());
    let mut tail = None;
    let ascending_start = ints.len()+1;
    let labels = ints.into_iter().chain(ascending_start..total+1);
    for value in labels.rev() {
        current = Rc::new(CupList::push(value, &current));
        cup_pointers[value] = Rc::clone(&current);
        if tail.is_none() {
            tail = Some(Rc::clone(&current));
        }
    }
    // create circular reference
    tail.unwrap().set_next(&current);
    (current, cup_pointers)
}

fn play(mut current: Rc<CupList>, cup_pointers: &Vec<Rc<CupList>>, moves: u32) -> Rc<CupList> {
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
        let mut d = if c == 1 { cup_pointers.len()-1 } else { c - 1 }; 
        while d == c1 || d == c2 || d == c3 {
            d = if d == 1 { cup_pointers.len()-1 } else { d - 1 }; 
        };

        //3.
        let destination = Rc::clone(&cup_pointers[d]);
        cup3.set_next(&destination.next().unwrap());
        destination.set_next(&cup1);
        
        current = current.next().unwrap();
    }

    current
}
