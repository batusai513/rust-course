/*
    enum Result<T, E> {
        Ok(T),
        Err(R)
    }
*/

#[derive(Debug)]
enum ErrorDivision {
    DivisionPorCero,
    DivisionNegativos,
}

fn division(dividendo: i32, divisor: i32) -> Result<i32, ErrorDivision> {
    if divisor == 0 {
        return Err(ErrorDivision::DivisionPorCero);
    }

    if dividendo < 0 || divisor < 0 {
        return Err(ErrorDivision::DivisionNegativos);
    }

    Ok(dividendo / divisor)
}

fn main() {
    // Result, permite trabajar con errores

    // Result tambien tiene los metodos unwrap, unwrap_or, except, funcionan igual a los metodos de Option

    let resultado = division(10, 0);

    match resultado {
        Ok(valor) => println!("el resultado es {}", valor),
        Err(ErrorDivision::DivisionPorCero) => {
            println!("El error es por intentar dividir entre cero")
        }
        Err(ErrorDivision::DivisionNegativos) => {
            println!("El error es por intentar dividir numeros negativos")
        }
    }

    // println!("el resultado es {:?}", resultado);
}
