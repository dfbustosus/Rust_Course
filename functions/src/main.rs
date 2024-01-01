/* 
fn main() {
    println!("Hello, world!");
    otra_funcion();
}

fn otra_funcion(){
    println!("Otra funcion")
}

*/

/* 
fn main(){
    fun_2(5);
}

fn fun_2(x:i32){
    println!("EL valor de x es: {x}");
}
*/

/* 
fn main(){
    print_david(5,'D');
}

fn print_david(valor:i32, unidad:char){
    println!("El valor es: {valor}{unidad}")
}
*/

/* 
fn cinco()-> i32{
    //return 5
    5
}

fn main(){
    let x= cinco();
    println!("El valor de x es: {x}");
}
*/

fn main(){
    let x= mas_uno(10);
    println!("El valor de x es: {x}")
}

fn mas_uno(x:i32) -> i32 {
    x+1
}