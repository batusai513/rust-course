fn main() {
    let mensaje = "Variable en el bloque main";

    {
        let mensaje = "shadowing variable en bloque anidado";
        let mensaje_dos = "Variable bloque anidado";

        println!("{}, {}", mensaje, mensaje_dos);
    }

    println!("{}", mensaje);

    // los bloques retornan valores

    let resultado = {
        println!("Segundo bloque anidado");
        let variable: i32 = 200;
        println!("{}", variable);
        variable
    };

    println!("el resultado es: {}", resultado);

    let calificacion: i8 = 10;

    let mensaje = if calificacion == 10 {
        String::from("Felicidades 🚀")
    } else {
        String::from("Estudia mas")
    };

    println!("{}", mensaje);
}
