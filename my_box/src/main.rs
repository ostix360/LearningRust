use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T>{
    fn drop(&mut self) {
        println!("Cleaning up");
    }
}

fn hi(name: &str){
    println!("Hi, {}!", name);
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    drop(x);
    let name = MyBox::new(String::from("Rust"));
    hi(&name);
}
