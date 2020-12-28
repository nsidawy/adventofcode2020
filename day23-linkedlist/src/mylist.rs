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

    pub fn new(value: usize, next: &Rc<CupList>) -> Self {
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
            CupList::Cup(_, next) => Some(Rc::clone(&*next.borrow()))
        }
    }

    pub fn set_next(&self, next: &Rc<CupList>) {
        match self {
            CupList::Cup(_, n) => { *n.borrow_mut() = Rc::clone(&next); },
            CupList::Nil => panic!("Can't add next to Nil node")
        }
    }

    pub fn iter(&self) -> CupListIter {
        CupListIter { start_value: self.value().unwrap_or(0), current: Rc::new(self.clone()), is_start: false }
    }

    pub fn count(&self) -> usize {
        CupList::iter(self).count()
    }

    pub fn print(&self) {
        for cur in self.iter() {
            if let CupList::Cup(i,_) = *cur {
                print!("{},", i);
            }
        }
        println!("");
    }
}

pub struct CupListIter {
    current: Rc<CupList>,
    start_value: usize,
    is_start: bool
}

impl Iterator for CupListIter {
    type Item = Rc<CupList>;

    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> { 
        if let CupList::Nil = *self.current {
            return None;
        }
        if self.current.value().unwrap() == self.start_value && self.is_start {
            return None;
        }
        self.is_start = true;
        let current = Rc::clone(&self.current);
        self.current = Rc::clone(&current.next().unwrap());
        Some(current)
     }
}
