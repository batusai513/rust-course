fn main() {
    // // bloque 1
    // // el bloque limita el scope de una variable
    // let mensaje = "Hola, soy una variable del bloque main";

    // println!("bloque 1 {}", mensaje);

    // {
    //     //bloque 2
    //     let mensaje = "Hola soy una variable del bloque 2";
    //     println!("bloque 2 {}", mensaje);

    //     {
    //         // bloque 3
    //         println!("bloque 3 {}", mensaje);
    //     }
    // }

    //prestamos

    let mut mensaje = String::from("Hola, soy una variable para prestamo");

    {
        let prestamo = &mensaje; // Prestamo, mensaje se mueve de main a anidado

        mensaje = String::from("Cambio de valor"); // freezing, no se puede modificar una referencia prestada y que luego sera usada

        println!("{}", prestamo);
    }

    println!("{}", mensaje);
}
