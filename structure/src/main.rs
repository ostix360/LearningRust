struct User {
	username: String,
	email: String,
	psw: String,
	online: bool
}

fn main() {
	let mut u1 = User{
		username: String::from("Bob"),
		online: true,
		psw: String::from("1234"),
		email: String::from("bob@bob.bob")
	};
	
    println!("{}",u1.email);

	u1.email = String::from("boby@boby.bob");
	
	println!("{}", u1.email);
	
	let u2 = create_user("barbara".to_string(), "456".to_string(), "barbara@boby.bob".to_string());
	println!("{}",u2.email);
	
	let u3 = User {
		online: false,
		..u2
	};
	
	println!("l'utilisateur {} est il en ligne? {}", u3.username, u3.online);
}

fn create_user(username: String, psw: String, email: String) -> User{
	User {
		username,
		psw,
		email,
		online: true,
	}
}
