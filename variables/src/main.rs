use std::io;
fn main() {
    // Variables
    let mut x=5;
    println!("El valor de x es: {x}");
    x=6;
    println!("El valor de x es: {x}");
    // Constantes
    const TRES_HORAS: u32 = 60*60*3;
    println!("La constante es: {TRES_HORAS}");
    // Shadowing (Otra forma de hacer el mut)
    let y= 10;
    let y= y+1;

    {
        let y= y+2;
        println!("El valor final de y es: {y}")
    }
    // Data types (scalar and compound)
    // 1. Scalar
    // 1.1 Integer
    /* 4 tipos 
       1.1.1 Integers >> 8,16,32,64,128 bits almacenando -(2^(n-1)) hasta (2^(n-1))-1
       1.1.2 Floating >> f32 y f64
       1.1.3 Boolean >> true, false
       1.1.4 Charcaters >> 4bits caracteres unicos 
     */
    // 1.1.1
    let c:i8 = -1; //signed
    let a:u8 = 1; // unsigned
    println!("El valor de a es {a}");
    let b:u64 = 134566; // unsigned 64bits
    println!("El valor de a es {a}");
    println!("El valor de b es {b}");
    println!("El valor de c es {c}");
    // 1.1.2
    let d = 2.0; //f64
    let e:f32 = 3.0;// f32
    // Operaciones
    let sum = 5+1;
    let dif = 95.3-4.2;
    let prod = 3 * 20;
    let quot= 52.3/43.1;
    let trunc = -5/3;
    let resto= 45 % 4;
    println!("sum = {sum}, dif={dif}, prod={prod}, quot={quot}, trunc={trunc}, resto={resto}");
    // 1.1.3
    let t =true;
    let f:bool = false;
    println!("t = {t}, f={f}");
    // 1.1.4
    let character = 'd';
    let z:char = 'D';
    let cat = '😻';
    println!("cat = {cat}");
    // 2. Compound
    /* 2 tipos
        2.1.1 Tuples >>  tipo de dato distinto de tamaño fijo
        2.1.2 Arrays >> Longitud fija tipo de dato unico
     */
    // 2.1.1
    let tup:(i32, f64,u8) = (500,6.4,1);
    println!("Tuple {:?}", tup);
    let tup2 = (500, 5.1, 1);
    let (x,y,z) = tup;
    println!("El valor de y es: {y}");
    // Acceeso a elementos de tupla
    let tup3: (i32, f64, u8)= (500, 50.3, 10);
    let a1 = tup3.0;
    let a2 = tup3.1;
    let a3= tup3.2;
    println!("a1 ={a1}, a2 ={a2}, a3 ={a3}");
    // 2.1.2
    let array = [1,2,3,4,5];
    let meses = ["Ene","Feb","Mar","Abr","May","Jun","Jul","Ago","Sep",
                "Oct","Nov","Dec"];
    println!("meses: {:?}", meses);          
    let array2: [i32; 3]= [1,4,6]; // tipo de dato; tamaño
    let primero = array2[1];
    println!("array2: {:?}", array2);          
    let array3 =[3;5]; // 5 veces 3 
    println!("array3: {:?}", array3);    
    // Indexador 
    let array4= [1,3,5,7,9];
    println!("Ingresa un indice del array");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Fallando leer la linea");

    let index:usize =index
        .trim()
        .parse()
        .expect("Indice no es un numero");
    
    let elemento = array4[index];
    println!("El valor elegido del indice {index} es : {elemento}"); 
         
}
