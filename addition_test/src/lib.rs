#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn ajouter_deux(a: i32) -> i32 {
    addition_interne(a, 2)
}

fn addition_interne(a: i32, b: i32) -> i32 {
    a + b
}


impl Rectangle {
    fn contain(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn welcomed(name: &str) -> String {
    format!("Salut, {} !", name)
}

pub fn panic_panic() -> bool {
    if true {
        panic!("Pas de panic à bord...")
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_can_contain_smaller() {
        let large: Rectangle = Rectangle { width: 8, height: 7 };
        let small = Rectangle { width: 4, height: 4 };

        assert!(large.contain(&small));
    }

    #[test]
    #[ignore]
    fn small_cant_contain_large() {
        let large: Rectangle = Rectangle { width: 8, height: 7 };
        let small = Rectangle { width: 4, height: 4 };

        assert!(!small.contain(&large));
    }

    #[test]
    fn welcome() {
        let res = welcomed("Joe");
        assert!(res.contains("Joe"),
                "Le message doit contenire le nom mais il vaut seulement  '{}' ", res);
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        panic_panic();
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            return Ok(());
        }
        Err("2 + 2 ne vaut pas 4".to_string())
    }
}
