// io = input/output library 
// fn representa function
use std::io;
use rand::Rng; // dependencia externa definida en el Cargo.toml
use std::cmp::Ordering;
fn main() {
    println!("Adivina tu numero!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // Borrar el num para que la gente no lo vea
    //println!("El numero secreto es: {secret_number}");
    
    loop {
        println!("Ingresa tu guess");
        // variable guess con let (variables son inmutables>> no cambian )
        // PERO CON EL mut se vuelven mutablees
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Fallo al leer la linea");
        // Asegurarse que sea un numero variable guess
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Has elegido: {guess}");
        // Comparacion
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Mas pequeno"),
            Ordering::Greater => println!("Mas grande"),
            Ordering::Equal => {
                println!("Ganaste");
                break; // Salida programa
            }
        }
            
    }
}
