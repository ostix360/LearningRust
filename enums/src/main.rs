#[derive(Debug)]
enum IPAdress{
	V4(u8, u8, u8, u8),
	V6(String)
}

impl IPAdress{
	fn request(&self,dest: &IPAdress) -> bool{
		false//TODO 
	}
}

fn main() {
	let local = IPAdress::V4(127, 0, 0, 1);
    println!("Local : {:#?}", local);
}


// enum IPAdress{
	// V4(String),
	// V6(String)
// }

// enum IPAdressVersion{
	// V4,
	// V6
// }
// struct IPAdress{
	// version: IPAdressVersion,
	// adress: String
// }

