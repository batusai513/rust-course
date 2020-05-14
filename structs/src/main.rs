// se almacenan en el HEAP
struct User {
    username: String,
    password: String
}

fn create_user(username: String, password: String) -> User {
    User { username, password }
}

fn main() {
    let usuario: User = User {
        username: String::from("usuario"),
        password: String::from("123")
    };

    let usuario_dos = create_user(String::from("wardrakus"), String::from("1234567"));

    println!("el username es: {}", usuario_dos.username);
    println!("el password es: {}", usuario_dos.password);
}
