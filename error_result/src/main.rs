use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // let f = File::open("file.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("file.txt") {
    //             Ok(fc) => fc,
    //             Err(err) => panic!("Erreur de création du fichier : {:?}", err),
    //         },
    //         others_error => panic!("Erreur d'ouverture du fichier : {:?}", others_error)
    //     }
    // };

    let f = File::open("file.txt").unwrap_or_else(|err|{
        if err.kind() == ErrorKind::NotFound {
            File::create("file.txt").unwrap_or_else(|err|{
                panic!("Erreur de création du fichier : {:?}", err);
            })
        }else{
            panic!("Erreur d'ouverture du fichier : {:?}", err);
        }
    });

}
