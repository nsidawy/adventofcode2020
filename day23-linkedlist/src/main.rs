use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cup(usize, RefCell<Rc<List>>),
    Nil
}

impl List {
    pub fn get_rc_nil() -> Rc<List> {
        Rc::new(List::Nil)
    }

    pub fn value(&self) -> Option<usize> {
        match self {
            List::Nil => None,
            List::Cup(v, _) => Some(*v)
        }
    }

    pub fn next(&self) -> Option<Rc<List>> {
        match self {
            List::Nil => None,
            List::Cup(_, next) => Some(Rc::clone(&*next.borrow()))
        }
    }

    pub fn set_next(&self, next: &Rc<List>) {
        if let List::Cup(_, n) = self {
            *n.borrow_mut() = Rc::clone(&next);
        }
    }
}

fn main() {
    //let input = "389125467";
    let input = "123487596";
    let total = input.len();
    let (curr, next) = get_cups(input, total);
    let curr = play(curr, &next, 100);
    println!("{}", count(&curr));
    print(&curr);

    let total = 1000000;
    let (curr, next) = get_cups(input, total);
    println!("{}", count(&curr));
    let _ = play(curr, &next, 10000000);
    let start = Rc::clone(&(next[1]));
    println!("{}", count(&start));
    let n1 = start.next().unwrap();
    let n2 = n1.next().unwrap();
    let v1 = n1.value().unwrap();
    let v2 = n2.value().unwrap();
    println!("{}", v1*v2);
}

fn count(next: &Rc<List>) -> usize {
    let start = if let List::Cup(i,_) = **next { i } else { 0 };
    let mut current = Rc::clone(&*next);
    let mut count = 0;
    while let List::Cup(_,next) = &*Rc::clone(&current) {
        count += 1;
        current = Rc::clone(&*next.borrow());
        if let List::Cup(i,_) = *Rc::clone(&current) {
            if i == start {
                break;
            }
        }
    }
    count
}

fn print(next: &Rc<List>) {
    let start = if let List::Cup(i,_) = **next { i } else { 0 };
    let mut current = Rc::clone(&*next);
    while let List::Cup(i,next) = &*Rc::clone(&current) {
        print!("{}", i);
        current = Rc::clone(&*next.borrow());
        if let List::Cup(i,_) = *Rc::clone(&current) {
            if i == start {
                break;
            }
        }
    }
    println!("");
}

fn get_cups(input: &str, total: usize) -> (Rc<List>, Vec<Rc<List>>) {
    let ints: Vec<usize> = input.chars().map(|c| (c.to_digit(10).unwrap()) as usize).collect();
    let mut next = vec![List::get_rc_nil(); total + 1];
    let mut current = List::get_rc_nil();
    let mut first = List::get_rc_nil();
    for i in (ints.len()+1..total+1).rev().chain((0..ints.len()).rev()) {
        let value = if i < ints.len() { ints[i] } else { i };
        current = Rc::new(List::Cup(value, RefCell::new(Rc::clone(&current))));
        next[value] = Rc::clone(&current);
        if let List::Nil = *Rc::clone(&first) {
            first = Rc::clone(&current);
        }
    }
    first.set_next(&current);
    (current, next)
}

fn play(mut current: Rc<List>, next: &Vec<Rc<List>>, moves: u32) -> Rc<List> {
    for x in 0..moves {
        //1.
        let cup1 = current.next().unwrap();
        let cup2 = cup1.next().unwrap();
        let cup3 = cup2.next().unwrap();
        current.set_next(&cup3.next().unwrap());
    
        //2.
        if let List::Nil = *current { println!("NIL {}", x) }
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
