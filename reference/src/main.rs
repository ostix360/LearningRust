fn main() {
    let mut s = String::from("Hello, world!");
	
	let s1 = &s;
	let s2 = &s;
	
	println!("La longueur de \"{}\" est {}.",s,string_length(&s));
	println!("{}, {}",s1,s2);
	
	let s3 = &mut s;
	chang(s3);
	println!("Variable modifié, {}",s3);
}

fn chang(s :&mut String){
	s.push_str(" I'm happy to see you");
}

fn string_length(s: &String) -> usize {
	s.len()
}
