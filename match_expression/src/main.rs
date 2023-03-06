#[derive(Debug)]
enum IPAdress{
	V4(u8, u8, u8, u8),
	V6(String)
}

impl IPAdress{
	// fn request(&self,dest: &IPAdress) -> bool{
		// match self{
			// IPAdress::V4(a, b, c, d) => {
								// match dest{
									// IPAdress::V4(f, g, h, i) => a == f && b == g && c == h,
									// _ => false
								// }
							// },
			// _ => false
		// }
	// }
	
	fn request(&self,dest: &IPAdress) -> bool{
		if let IPAdress::V4(a, b, c, d) = self{
			if let IPAdress::V4(f, g, h, i) = dest{
				true
			}else{
				false
			}
		}else{
			false
		}
	}
}

fn main() {
	let local = IPAdress::V4(127, 0, 0, 1);
	
    println!("local is localhose? {}", local.request(&IPAdress::V4(127, 0, 0, 1)));
}
