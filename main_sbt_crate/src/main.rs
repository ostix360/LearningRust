fn main() {
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
	
}
