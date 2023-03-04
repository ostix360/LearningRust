#[derive(Debug)]
struct Rectangle{
	width: u32,
	height: u32
}

fn main() {
	let width: u32 = 3;
	let height: u32 = 4;
	
	// let dimensions = (width, height);
	
	let rect = Rectangle{
		width,
		height
	};
		
    println!("L'aire d'un rectangle de {} de longeur, et de {} de largeur est {}", width, height, aire(&rect));
	
	dbg!(&rect);
}

// fn aire(width: u32, height: u32) -> u32{
	// width * height
// }

// fn aire(dimensions: (u32, u32)) -> u32{
	// dimensions.0 * dimensions.1
// }

fn aire(rectangle: &Rectangle) -> u32{
	rectangle.width * rectangle.height
}
