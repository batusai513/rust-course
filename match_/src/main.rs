fn main() {
    let numero: i32 = 8;

    let mensaje = match numero {
        1 => "numero uno",
        2 => "numero dos",
        3 => "numero tres",
        4 | 5 | 6 => "numero esta entre cuatro y seis",
        7..=100 =>  "el numero se evalua mediante un rango del 7 al 100",
        _ => "numero"
    };

    println!("el resultado es {}", mensaje);


    //fizzbuzz

    for numero in 1..31 {
        match (numero % 3, numero % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", numero)
        }
    }
}
