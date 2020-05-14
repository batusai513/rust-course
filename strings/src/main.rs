fn main() {
    // str -> cadena de caracteres inmutable, se almacena en el stack
    // String -> cadena mutable de caracteres, se almacena en el heap

    let variable_str = "tipo str";
    // let variable_string = String::new(); //string vacio
    let mut variable_string = String::from("Cadena string");

    variable_string.push(',');
    variable_string.push(' ');
    variable_string.push('H');
    variable_string.push('O');
    variable_string.push('L');
    variable_string.push('A');

    variable_string.push_str(" Estamos en el curso de rust");

    let nuevo_string = "Hola soy una cadena".to_string();

    let igual = nuevo_string == "Hola soy una cadena.".to_string();

    println!("str: {}, String: {}", variable_str, variable_string);
    println!("{}", nuevo_string);
    println!("{}", igual);
}
