// se almacenan en el HEAP
#[derive(Debug)]
struct User {
    username: String,
    password: String
}

impl User {
    fn saluda(&self) {
        println!("Hola soy el usuario {}", self.username);
    }

    fn cambiar_password(&mut self, password: String) {
        self.password = password;
    }
}

fn main() {
    let mut usuario: User = User {
        username: String::from("usuario"),
        password: String::from("123")
    };

    usuario.saluda();

    println!("{:?}", usuario);
    usuario.cambiar_password(String::from("Nuevo Password"));

    println!("{}", usuario.username);
    println!("{:?}", usuario);

}
