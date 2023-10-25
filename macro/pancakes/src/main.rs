use my_macro::MyMacro;
use my_macro_derive::MyMacro;

#[derive(MyMacro)]
struct Pancakes;

fn main() {
    Pancakes::my_macro();
}