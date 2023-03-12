mod acceuil;


pub mod salle_a_manger{
	use crate::salle_a_manger::acceuil::acceuil::ajouter_a_la_file;

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