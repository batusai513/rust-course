fn saludar_usuarios() {
    println!("Hola desde la funcion");
}

fn sumar(numero_uno: i32, numero_dos: i32) -> i32 {
    numero_uno + numero_dos
}

fn factorial(numero: u32) -> u32 {
    if numero == 1 {
        return numero;
    }
    factorial(numero - 1) * numero
}

fn main() {
    saludar_usuarios();
    println!("el resultado es {}", sumar(1, 2));
    println!("el factorial es {}", factorial(5));
}
