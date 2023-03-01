fn main() {
    let mut s = String::from("Hello");
	
	s.push_str(", world!");
	
	let s2 = s;
	
	prend(s2);
	
	
	let s3 = String::from("Bye everyone!");
	
	prend(juste_un_tour(s3));
	
}

fn prend(s:String){
	println!("{}",s);
}

fn juste_un_tour(s:String) -> String{
	s
}
