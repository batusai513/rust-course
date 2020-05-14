fn main() {
    let mensaje = None;

    match mensaje {
        Some("Hola mundo") => println!("el mensaje es hola mundo"),
        Some("Adios") => println!("El mensaje es adios"),
        Some(_) => println!("Es otro mensaje"),
        None => println!("No existe valor alguno"),
    }
}
