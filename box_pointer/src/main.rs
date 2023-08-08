use std::cell::RefCell;
use std::rc::Rc;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let value = Rc::new(RefCell::new(1));
    let list = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let a = Cons(Rc::new(RefCell::new(0)), Rc::clone(&list));
    let b = Cons(Rc::new(RefCell::new(-1)), Rc::clone(&list));

    *value.borrow_mut() += 10;

    println!("list after op = {:?}", list);
    println!("a after op = {:?}", a);
    println!("b after op = {:?}", b);
}
