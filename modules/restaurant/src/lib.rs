pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod Cuisine{
	pub struct Breakfirst{
		pub pain: String,
		fruit: String
	}
	
	impl Breakfirst{
		pub fn in_summer(pain: &String) -> Breakfirst{
			Breakfirst{
				pain: String::from(pain),
				fruit: String::from("peach")
			}
		}
	}
}

use Cuisine::Breakfirst as breakfirst;

pub fn order_breakfirst(){
	let mut breakfirst = breakfirst::in_summer(&String::from("pain de mie"));
	
	
	breakfirst.pain = String::from("blé");
	
	//illégale
	
	//breakfirst.fruit = String::from("fraise");
}

mod salle_a_manger{
	pub mod acceuil{
		pub fn ajouter_a_la_file(){
			println!("Dans la files");
		}
		
		fn installer(){}
	}
	
	fn manger(){
		crate::salle_a_manger::acceuil::ajouter_a_la_file();
		
		super::salle_a_manger::acceuil::ajouter_a_la_file();
	}
	
	mod service{
		fn prendre_commande(){}
		
		fn servir(){}
		
		fn encaisser(){}
	}
}