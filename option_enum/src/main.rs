fn main() {
	let a = Some(3);
	let b = Some(2);
	let c = add(a, b);
    println!("La somme de {:?} plus {:?} est {:?}", a, b, c);
}

fn add(a: Option<i32>, b: Option<i32>) -> Option<i32>{
	if let Some(a_) = a {
		if let Some(b_) = b{
			Some(a_ + b_)
		}else{
			None
		}
	}else{
		None
	}
}
