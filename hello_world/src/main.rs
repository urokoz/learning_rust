use std::io;

fn main() {
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Could not read name.");

    let name = name.trim();

    if name == "Sam" {
        println!("Hola sexy");
    } else {
        println!("Hello, {name}!")
    }
}
