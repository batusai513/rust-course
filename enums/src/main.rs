fn main() {
    // Enum -> CamelCase
    enum Response {
        Success,
        Error(u32, String), // 403, 404, 500, etc...
    }

    let respuesta = Response::Error(600, String::from("No es posible proceder"));

    match respuesta {
        Response::Success => println!("la peticion se realizo exitosamente"),
        Response::Error(403, _) => println!("Forbiden"),
        Response::Error(404, _) => println!("Not Found"),
        Response::Error(500, _) => println!("Internal server error"),
        Response::Error(_, mensaje) => println!("{}", mensaje)
    }
}
