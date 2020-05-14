fn main() {
    //Slices, no se les conoce el tamanio en tiempo de ejecucion, almacenados en el stack,
    // Arrays => Stack

    let mensaje = String::from("Hola mundo desde rust");

    let hola = &mensaje[..4]; // [start..end]; [0..4]

    let resto_mensaje = &mensaje[4..];

    let mensaje_comleto = &mensaje[..];

    println!("el mensaje es: {}", mensaje);
    println!("el slice es: {}", hola);
    println!("el resto es: {}", resto_mensaje.trim());
    println!("todo es: {}", mensaje_comleto);

}
