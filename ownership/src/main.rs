fn main() {
    println!("Hello, world!");
    let mut s = String::from("Hola");
    s.push_str(", mundo!");

    println!("{}", s);

    // Ejemplo 2
    let x= 5;
    let y= x;
    println!("{y}");
    // Ejemplo 2
    let s1 = String::from("hola");
    let s2 = s1; // Aqui s1 y s2 estarian apuntando a lo mismo y no se puede hacer el drop natural de una variable y otra
    //println!("{}, mundo", s1); // La variable s1 queda deshabilitada
    println!("{}, mundo", s2);
    // Rust no hace deep copying para garantizar performance

    // Si quieres hacer un deep copying puedes usar metodo clone
    let s3 = String::from("hola");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}",s3,s4);

    // Ownership de funciones
    let s5 = String::from("David");
    tomando_owner(s5); // s5 pasa al scope de la funcion
    //println!("{s5}");// No puede ser usado aqui
    let x3= 5;
    hacer_copia(x3); // Aqui no pasa nada los int se pueden copiar

    // Transferencia de ownership 
    //let s6 = dar_owner();
    //let s7 = String::from("Hola");
    //let s8 = tomar_y_devolver_owner(s7);
    //println!("{s8}");


}

fn tomando_owner(string:String){
    println!("{}", string);
}

fn hacer_copia(entero: i32){
    println!("{}", entero);
}

/* 
fn dar_owner() -> String {
    let string_david = String::from("Tu");
    string_david // Return
}

fn tomar_y_devolver_owner(un_string: String) -> String{
    un_string // Return
}
*/