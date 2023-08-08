use std::rc::Rc;
use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    println!("Hello, world!");
    let list = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(
                Cons(3, Rc::new(Nil)))))));
    println!("Count after creation of list {}", Rc::strong_count(&list));
    let a = Cons(0, Rc::clone(&list));
    println!("Count after creation of a {}", Rc::strong_count(&list));
    {
        let b = Cons(-1, Rc::clone(&list));
        println!("Count after creation of b {}", Rc::strong_count(&list));
    }
    println!("Count after b is out of scope {}", Rc::strong_count(&list));
}
