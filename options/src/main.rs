/*
enum Option<T> {
    Some(T), -> el valor
    None -> ausencia de valor
}
*/

fn obtener_valor(bandera: bool) -> Option<String> {
    if bandera == true {
        Some(String::from("mensaje de la tupla some"))
    } else {
        None
    }
}

fn main() {
    // en rust no existen tipos que representen la ausencia de algun valor, null, nil, undefined

    // para eso existen estos enums especiales
    // Option -> Existe un valor o no
    // Result Errores -> panic!
    let resutado = obtener_valor(false); //Option

    match &resutado {
        Some(value) => println!("resultado: {}", value),
        None => println!("no existe valor alguno"),
    }

    // obtener el valor del option por medio de metodos (unwrap, unwrap_or, expect)
    // unwrap: intenta obtener lo que la tupla some almacena

    let valor = resutado.unwrap_or(String::from("la tupla no almacena valor alguno"));

    println!("el valor es: {}", valor);

    #[derive(Debug)]
    struct User {
        name: String,
        password: String,
        email: String,
        edad: Option<u32>,
    }

    let usuario_uno = User {
        name: String::from("Richard"),
        password: String::from("1234"),
        email: String::from("asd@asd.com"),
        edad: None
    };

    // let edad = usuario_uno.edad.unwrap();

    match usuario_uno.edad{
        Some(edad) => println!("edad {}", edad),
        None => {}
    }

    // println!("el usuario es: {:?}, la edad es: {}", usuario_uno, edad);
}
