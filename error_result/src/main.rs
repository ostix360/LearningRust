use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};
use std::fs;

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

    let s = match read_file() {
        Ok(s) => s,
        Err(e) => panic!("Erreur ah {:?}", e)
    };

    println!("{}", s)
}


fn read_file() -> Result<String, io::Error>{
    let mut f = File::open("file.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

    // Ou

    // fs::read_to_string("file.txt");
}
