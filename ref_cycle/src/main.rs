use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List{
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

impl List {
    fn go_trough(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, item) => {Some(item)}
            List::Nil => None
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parents: RefCell<Weak<Node>>,
    childs: RefCell<Vec<Rc<Node>>>,
}



fn main(){
    let leaf = Rc::new(Node {
        value: 3,
        parents: RefCell::new(Weak::new()),
        childs: RefCell::new(vec![])
    });

    println!("parent de la feuille = {:?}", leaf.parents.borrow().upgrade());

    let branch = Rc::new(Node{
        value:5,
        parents: RefCell::new(Weak::new()),
        childs: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parents.borrow_mut() = Rc::downgrade(&branch);

    println!("parent de la feuille = {:?}", leaf.parents.borrow().upgrade());

    // let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    //
    // println!("compteur initial de a = {}", Rc::strong_count(&a));
    // println!("prochain élément de a = {:?}", a.go_trough());
    //
    // let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    //
    // println!("compteur de a après création de b = {}", Rc::strong_count(&a));
    // println!("compteur initial de b = {}", Rc::strong_count(&b));
    // println!("prochain élément de b = {:?}", b.go_trough());
    //
    // if let Some(lien) = a.go_trough() {
    //     *lien.borrow_mut() = Rc::clone(&b);
    // }
    //
    // println!("compteur de b après avoir changé a = {}", Rc::strong_count(&b));
    // println!("compteur de a après avoir changé a = {}", Rc::strong_count(&a));
    //
    // // Décommentez la ligne suivante pour constater que nous sommes dans
    // // une boucle de références, cela fera déborder la pile
    // println!("prochain élément de a = {:?}", a.go_trough());
}
