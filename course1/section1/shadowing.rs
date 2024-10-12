fn main(){
    let _an_32_integer:i32=5;
    let _an_64_integer:i64=37;
    let _an_32_float:f32=1.5;
    let _an_64_float:f64=31.5;
    let _another_64_float:f64=11.5;

    let a = 10;
    println!("{}",a);
    let a = 20;
    println!("{}",a);

    let an_variable:i32=5;
    println!("an_variable as integer {}",an_variable);

    let an_variable:char='c';
    println!("an_variable as  char {}",an_variable);


 
    let precio = "10";
    let precio: i32 = precio.parse().expect("No es un número");
    let precio_con_iva = precio + (precio * 21 / 100);
    println!("El precio con IVA es: {}", precio_con_iva);


    
    let x = 10;
    println!("Fuera del ámbito, x = {}", x); // x vuelve a ser 10
    {
        let x = x * 2; // x dentro del ámbito es 20
        println!("Dentro del ámbito, x = {}", x);

        let x = 'C'; // x dentro del ámbito es C
        println!("Dentro del ámbito ahora soy un caracter, x = {}", x);
        {
            let x = 3.1454; // x dentro del ámbito es 20
            println!("Dentro del sub ámbito, ahora soy 3.14 x = {}", x);
        }
        println!("Afuera del sub ámbito, x = {}", x);
    }
    println!("Fuera del ámbito, x = {}", x); // x vuelve a ser 10

    
    
    
}