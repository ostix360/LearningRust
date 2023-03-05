#[derive(Debug)]
struct Rectangle{
	height: u32,
	width: u32,
}

impl Rectangle{
	fn aire(&self) -> u32{
		self.height * self.width
	}
	
	fn can_contain(&self, other: &Rectangle) -> bool{
		self.height >= other.height && self.width >= other.width
	}
	
	fn square(size: u32) -> Rectangle{
		Rectangle{
			height: size,
			width: size
		}
	}
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40
    };
    let square = Rectangle::square(35);
	
	
	println!("L'aire de rect1 est {}", rect1.aire());
    println!("rect1 peut-il contenir rect2 ? {}", rect1.can_contain(&rect2));
    println!("rect1 peut-il contenir square ? {}", rect1.can_contain(&square));
}
