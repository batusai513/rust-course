fn main() {

    let mut contador = 0;
    // loop
    loop {
        contador += 1;
        println!("contador: {}", contador);

        if contador == 10 {
            break;
        }
    }


    //for
    let numeros: [i32; 5] = [1,2,3,4,5];

    for numero in numeros.iter() {
        println!("{}", numero);
    }

    for numero in 1..100 {
        println!("{}", numero);
    }


    //fizzbuzz

    for numero in 1..101 {
        if numero % 3 == 0 && numero % 5 == 0 {
            println!("FizzBuzz");
        } else if numero % 5 == 0 {
            println!("Buzz");
        } else if numero % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", numero);
        }
    }

    //ciclo while
    // while <condicion> {}

    //cantidad de digitos de un numero

    let mut numero = 1189000;
    let mut contador = 0;
    while numero > 0 {
        numero = numero / 10;
        contador += 1;
    }

    println!("Cantidad de digitos de {}, son: {}", numero, contador);
}
