struct Countdown {
    compteur: u32,
}

impl Countdown {
    fn new(value: u32,) -> Countdown {
        Countdown {
            compteur: value,
        }
    }
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.compteur > 0{
            self.compteur -=1;
            Some(self.compteur)
        }else {
            None
        }
    }
}

fn main() {
    let compteur = Countdown::new(10);

    for val in compteur{
        println!("Value of the countdown {}", val)
    }

    let somme: u32 = Countdown::new(6)
            .zip(Countdown::new(6).skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 != 0)
            .sum();
    // 20 + 2 = 22
    println!("{}", somme)

}
