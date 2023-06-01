use std::io::{Error, ErrorKind};
use std::io;

pub enum Operator{
    Add,
    Sub,
    Mult,
    Div
}

pub struct Operation{
    operator: Operator,
    v1: f32,
    v2: f32
}

impl Operation{
    pub fn new(v1: f32, v2: f32,op :&str) -> Result<Operation,Error> {
        let operator = match op {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mult,
            "/" => Operator::Div,
            _ => return Err(Error::new(ErrorKind::Other, "Opérateur non trouvé"))
        };
        Ok(Operation{operator, v1, v2})
    }

    pub fn calcul(&self) -> Result<f32,Error>{
        match &self.operator {
            Operator::Add => Ok(self.v1 + self.v2),
            Operator::Sub => Ok(self.v1 - self.v2),
            Operator::Mult => Ok(self.v1 * self.v2),
            Operator::Div => if self.v2 == 0.0 {
                Err(Error::new(ErrorKind::Other, "Impossible de divisé par 0"))
            }else{
                Ok(self.v1 / self.v2)
            }
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut restart: bool = true;

    while restart{
        println!("Choisissez un nombre");
        let v1: f32 = ask_number();
        println!("Choisissez un deuxième nombre");
        let v2: f32 = ask_number();

        println!("Choisissez un opérateur (+,-,*,/)");
        let op = ask_operator();

        
        let operation: Operation = Operation::new(v1, v2, &op)?;

        let result: f32 = operation.calcul()?;
        println!("{} {} {} = {}", v1, &op, v2, result);
        println!("--------------------------------------");
        println!("Avez vous d'autre calcule à effectuer?");
        restart = ask_restart();
    }
    println!("Merci et au revoir!");
    Ok(())
}

fn ask_number() -> f32{
    loop{
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Error while reading user input");
        match s.trim().parse() {
            Ok(v) => return v,
            Err(_) => println!("Veuilliez rentrer un nombre valide")
        };
    }
}

fn ask_operator() -> String{
    loop{
        let mut op = String::new();
        io::stdin().read_line(&mut op).expect("Error while reading user input");
        let op = op.trim();
        match op {
            "+"|"-"|"*"|"/" => return op.to_string(),
            _ => println!("Choisissez un opérateur valide (+,-,*,/)"),
        }
    }
}

fn ask_restart() -> bool{
    loop {
        println!("0 oui 1 non");
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Error during the user input");
            
        let guess: u32 = match guess.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => {
                println!("Soyez serieux svp, Voulez vous recommencer?");
                continue;
            }
        };
        if guess == 0 {
            return true;
        }else if guess == 1{
            return false;
        }
    }
}

