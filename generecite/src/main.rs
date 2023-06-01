
struct Point<X1, Y1>{
    x: X1,
    y: Y1
}


impl<X1, Y1> Point<X1, Y1>{
    fn mix<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2>{
        Point{
            x: self.x,
            y: other.y
        }
    }
}


fn main() {
    let 
}
