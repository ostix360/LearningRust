
struct Point<X1, Y1>{
    x: X1,
    y: Y1
}


impl<X1, Y1> Point<X1, Y1>{
    fn mix<X2, Y2>(self, other: &Point<X2, Y2>) -> Point<X1, &Y2>{
        Point{
            x: self.x,
            y: &other.y
        }
    }
}




fn main() {
    let p1 = Point { x: "Hello", y: "World"};

    let p2 = Point {x: 5, y: 2};

    let p3 = p1.mix(&p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let l =vec![2,5,8,3,6,1,7,4,8,0,4];
    println!("Heighter nb: {}", heighter(&l));

    let c = vec!['c', 'à', 'x', 'q', 't'];
    println!("Heighter char: {}", heighter(&c));

}

fn heighter<T>(liste :&[T]) -> T 
where T: PartialOrd + Copy{
    let mut heighter = liste[0];
    for &e in liste{
        if e > heighter{
            heighter = e;
        }
    }
    heighter
}
