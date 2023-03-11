use rand::Rng;
use std::collections::HashMap;
// use std::{cmp::Ordering, io};
use std::io::{self, Write};


fn main() {
	let s = rand::thread_rng().gen_range(1..10);
    println!("{}", s);
}
