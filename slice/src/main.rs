fn main() {
	let s = String::from("Hello, world!");
	
	let word = premier_mot(&s);
	
	println!("Le premier mot est {}", word);
	
	let mon_litteral_de_chaine = "Hello, world!";

    // premier_mot() fonctionne avec les slices de littéraux de chaîne, qu'elles
    // soient partielles ou intégrales
    let mot = premier_mot(&mon_litteral_de_chaine[7..]); //7 inclue
	
	println!("Le premier mot est (7..) {}", mot);
	
    let mot = premier_mot(&mon_litteral_de_chaine[..]);
	
	println!("Le premier mot est (..) {}", mot);

    // Comme les littéraux de chaîne sont déjà des slices de chaînes,
    // cela fonctionne aussi, sans la syntaxe de slice !
    let mot = premier_mot(mon_litteral_de_chaine);

    println!("Le premier mot est () {}", mot);
	
	
	let a = [1, 2, 3, 4, 5];

	let slice = &a[1..3];  //3 exclue

	println!("{}",slice == &[2, 3]);
}

fn premier_mot(s: &str) -> &str {	//le parmatere s :&str permet de prendre en compte les &String et les &str
    let octets = s.as_bytes();

    for (i, &element) in octets.iter().enumerate() {
        if element == b' ' {
            return &s[..i]; //i exclue
        }
		if element == b',' {
			return &s[..i];
		}
    }

    &s[..]
}
