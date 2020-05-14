struct Rectangulo {
    ancho: u32,
    alto: u32
}

fn area_rectangulo(rectangulo: &Rectangulo) -> u32 {
    rectangulo.ancho * rectangulo.alto
}


fn main() {

    //Ownership
    /*
        Todos los valores que se almacenan en el HEAP van a mover el ownership cuando se haga una asignacion
        a diferencia de las que se almacenan en el Stack

        - Cada valor de rust tiene su propio ownership
        - solo puede existir un ownership a la vez
        - si un ownership sale de alcance, el valor se descartara
    */

    let rectangulo = Rectangulo { alto: 10, ancho: 20 };

    // los argumentos son pasados mediante prestamos, estamos cediendo el ownership
    //let resultado = area_rectangulo(rectangulo);

    // Argumentos prestado por referencia
    let resultado = area_rectangulo(&rectangulo);

    // objetos que se almacenan en el HEAP
    let nuevo_rectangulo = rectangulo; // movimiento de ownership, rectangulo mueve el ownership a nuevo_rectangulo

    println!("el area es {}", resultado);
    println!("el ancho y alto del rectangulo es: {} - {}", rectangulo.ancho, rectangulo.alto);
}
