mod acceuil;
use crate::salle_a_manger::acceuil::acceuil::ajouter_a_la_file;

pub mod salle_a_manger{
	pub fn manger(){
		ajouter_a_la_file();
		
		ajouter_a_la_file();
	}
	
	mod service{
		fn prendre_commande(){}
		
		fn servir(){}
		
		fn encaisser(){}
	}
}