use std::io;
fn main() {
    // 1. Loop
    /* 
    loop {
        println!("de nuevo"); // Infinito hasta que paremos
    }
    */

    let mut contador = 0;
    let resultado = loop{
        contador +=1;
        println!("{contador}");
        if contador == 10{
            break contador*2;
        }
    };
    println!("El resultado es {resultado}");
    println!("-----------------------------------");
    let mut cont2 =0;
    'conteo_up: loop{
        println!("contero = {cont2}");
        let mut resto =10;

        loop {
            println!("resto = {resto}");
            if resto == 9{
                break;
            }
            if cont2 == 2{
                break 'conteo_up;// Cerrar llop extenro
            }
            resto -= 1;
        }
        cont2 +=1;
    }
    println!("Fin contero = {cont2}");

    println!("-----------------------------------");
    // 2. While
    let mut num3 = 3;
    while num3 !=0 {
        println!("{num3}");
        num3 -= 1;
    }
    println!("Saliendo");
    println!("-----------------------------------");
    // 3. For
    let a = [10,20,30,40];
    let mut index =0;
    while index <a.len() {
        println!("El valor es: {}", a[index]);
        index +=1;
    }
    println!("-----------------------------------");
    // For each
    let b= [100,200,300,400,500,600,700];
    for ele in b {
        println!("El valor es {ele}");
    }
    // Count down
    println!("-----------------------------------");
    for number in (1..4).rev(){
        println!("{number}");
    }
    println!("Saliendo");
    // Ejercicio de temperaturas
    println!("Ingresa un flotante: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("No se pudo leer");
    
    // Parsear a flotante
    let celsius: f64 = match input.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Input invalido");
            return;
        }
    };
    // Convertir a F
    println!("Tu temp en F es: {:.2}", celsius*(9.0/5.0)+32.0);

    // Ejercicio fibonacci
    println!("Ingresa un valor de n para encontar seq Fibonacci");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error en leer la linea");
    let n:u32 = match input.trim().parse(){
        Ok(num) => num,
        Err(_)=> {
            println!("Input invalido");
            return;
        }
    };
    let resultado = fibonacci(n);
    println!("El valor {}th de Fibonacci es: {}", n, resultado);

    // Ejercicio 3
    let dias = [
        "primer", "segundo", "tercer", "cuarto", "quinto", "sexto", 
        "séptimo", "octavo", "noveno", "décimo", "undécimo", "duodécimo",
    ];

    let gifts = [
        "un patridge en un peral",
        "dos tortolas y",
        "tres gallinas francesas,",
        "cuatro pájaros que llaman,",
        "cinco anillos dorados,",
        "seis gansos poniendo huevos,",
        "siete cisnes nadando,",
        "ocho doncellas ordeñando,",
        "nueve damas bailando,",
        "diez señores saltando,",
        "once músicos tocando,",
        "doce tambores tocando,",
    ];
    println!("¡Bienvenido a 'Los Doce Días de Navidad'!");
    println!("");
    for day in 0..12 {
        println!("En el {} día de Navidad, mi amor verdadero me regaló:", dias[day]);

        for gift in (0..=day).rev() {
            if gift == 0 && day != 0 {
                print!("y ");
            }
            println!("{}", gifts[gift]);
        }
        println!("");
    }


}

// Ejercicio de Fibonacci
fn fibonacci (n: u32) -> u64 {
    if n==0 {
        return 0;
    } else if n==1 {
        return 1
    } else {
        return fibonacci(n-1) + fibonacci(n-2);
    }
}

