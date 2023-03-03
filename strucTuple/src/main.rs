struct Color(i32,i32,i32);
struct Vector(i32,i32,i32);

fn main() {
    let random_color = Color(35,86,146);
	let i = Vector(1,0,0);
	
	println!("Random color: r {}, g {}, b {}", random_color.0, random_color.1, random_color.2);
	
	println!("Vector i has as coordinate: x {}, y {}, z {}", i.0, i.1, i.2);
}
