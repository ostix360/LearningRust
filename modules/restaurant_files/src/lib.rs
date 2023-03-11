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
mod salle_a_manger;
pub use Cuisine::Breakfirst as breakfirst;
use crate::salle_a_manger::salle_a_manger::manger as eat;

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

pub fn manger(){
	eat();
}

pub fn order_breakfirst(){
	let mut breakfirst = breakfirst::in_summer(&String::from("pain de mie"));
	
	
	breakfirst.pain = String::from("blé");
	
	//illégale
	
	//breakfirst.fruit = String::from("fraise");
}

