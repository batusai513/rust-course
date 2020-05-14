
#[derive(Debug)]
struct Color(u32, u32, u32); //RGB

fn main() {

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    let mut custom_color = Color(187, 62, 104);

    custom_color.1 = custom_color.1 + 10;

    println!("{:?}", black);
    println!("{:?}", white);
    println!("{:?}", custom_color);
}
