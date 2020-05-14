fn main() {
    let mut vector = vec![1, 2, 3];

    vector.push(4);
    vector.push(5);

    vector.insert(0, -1);
    vector.insert(1, 0);

    vector.remove(vector.len() - 1);

    let primer_elemento = vector[0];

    let ultimo_elemento = vector.last().unwrap();

    println!("{:?}", vector);
    println!("primer elemento: {}, ultimo elemento: {}", primer_elemento, ultimo_elemento);




    let mut vector: Vec<i32> = Vec::new();

    vector.push(1);
    vector.push(2);
    vector.push(3);
    vector.push(4);

    println!("{:?}", vector);
    
}
