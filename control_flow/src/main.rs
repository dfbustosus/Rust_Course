// 1. IF expressions
fn main() {
    let numero = 3;
    if numero < 5 {
        println!("Es menor que cinco");
    } else {
        println!("Es mayor igual que cinco")
    }
    // Incorporar funcion
    prueba();
    // If an let 
    let condicion = true;
    let num = if condicion {5} else {10};
    println!("El valor de num es: {num}");
}

fn prueba(){
    let numero = 10;
    if numero %  4 ==0 {
        println!("Numero es divisible por 4");
    } else if numero % 3 ==0 {
        println!("Numero es divisible por 3");
    } else if numero % 2 ==0 {
        println!("Numero es divisible por 2");
    } else {
        println!("Numero no es divisible por 4, 3 o 2")
    }
}