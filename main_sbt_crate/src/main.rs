fn main() {

	vectors();
	utf8_str();
	hashmap();
}

fn vectors(){
	let mut v: Vec<i32> = Vec::new();
	let v2 = vec![1,2,3,4,5];
	v.push(6);

	v.push(8);

	let i = &v[0];
    println!("The 3rd element of v is {}!", i);

	let index = 5;
	match v2.get(index){
		Some(i) => println!("l'élélement a l'index {} est {}", index, i),
		None => println!("Pas d'élément trouvé a l'index {}", index)
	}

	// v.push(6);  //illegal
	// println!("{}", i) //utilisation de i alors que la ref a disparue a cause du push qui realloue la memoire
	for i in &v{
		println!("{}", i);
	}

	let mut v3 = vec![100, 32, 57];

	for i in &mut v3{
		*i += 50;
	}

	#[derive(Debug)]
	enum Cellule{
		Int(i32),
		Float(f64),
		Text(String)
	}
	let ligne = vec![
		Cellule::Text(String::from("bleu")),
	];
	println!("{:?}",ligne);
}

fn utf8_str(){
	let bonjour = String::from("السلام عليكم"); // Demonstartion de l'UTF8

	let mut s = String::new();
	s.push_str("Hello,");
	s.push_str(" world!");
	let s2 = String::from("Hello, ");
	let s3 = String::from("world!");
	let fs = s2 + &s3; //Impossibilité de réutilisé s1

	let s2 = String::from("Hello, ");
	let fs2 = format!("{}-{}", s2, s3);

	let first_letter = &s2[0..1];
	println!("first letter :{}", first_letter);

	for c in fs2.chars(){
		println!("{}", c);
	};

}

fn hashmap(){

}
