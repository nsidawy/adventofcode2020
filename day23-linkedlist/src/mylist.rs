use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug,Clone)]
pub enum CupList {
    Cup(usize, RefCell<Rc<CupList>>),
    Nil
}

impl CupList {
    pub fn new_nil() -> Self {
        CupList::Nil
    }

    pub fn push(value: usize, next: &Rc<CupList>) -> Self {
        CupList::Cup(value, RefCell::new(Rc::clone(next)))
    }

    pub fn value(&self) -> Option<usize> {
        match self {
            CupList::Nil => None,
            CupList::Cup(v, _) => Some(*v)
        }
    }

    pub fn next(&self) -> Option<Rc<CupList>> {
        match self {
            CupList::Nil => None,
            CupList::Cup(_,next) => Some(Rc::clone(&next.borrow()))
        }
    }

    pub fn set_next(&self, next: &Rc<CupList>) {
        match self {
            CupList::Cup(_, n) => { *n.borrow_mut() = Rc::clone(next); },
            CupList::Nil => panic!("Can't add next to Nil node")
        }
    }

    pub fn iter(&self) -> CupListIter {
        CupListIter { start_value: self.value().unwrap_or(0), current: Rc::new(self.clone()), is_start: false }
    }

    pub fn count(&self) -> usize {
        self.iter().count()
    }

    pub fn print(&self) {
        println!("{}", self.iter().map(|c| c.value().unwrap()).fold(String::new(), |s,c| s + &c.to_string() + ","));
    }
}

pub struct CupListIter {
    current: Rc<CupList>,
    start_value: usize,
    is_start: bool
}

impl Iterator for CupListIter {
    type Item = Rc<CupList>;

    fn next(&mut self) -> std::option::Option<Self::Item> { 
        // end the iterator if we hit a cycle or hit a Nil
        if let CupList::Nil = *self.current {
            return None;
        }
        if self.current.value().unwrap() == self.start_value && self.is_start {
            return None;
        }
        self.is_start = true;
        let current = Rc::clone(&self.current);
        self.current = current.next().unwrap();
        Some(current)
     }
}
