#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}


impl Rectangle {
    fn contain(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_can_contain_smaller(){
        let large:Rectangle = Rectangle { width: 8, height:7};
        let small = Rectangle { width: 4, height: 4};

        assert!(large.contain(&small));
    }

     #[test]
    fn small_cant_contain_large(){
        let large:Rectangle = Rectangle { width: 8, height:7};
        let small = Rectangle { width: 4, height: 4};

        assert!(!small.contain(&large));
    }
}
