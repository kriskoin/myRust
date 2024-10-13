use std::io;
fn main (){
    let mut hilera = String::new();
    println!("Enter a String");
    io::stdin().read_line(&mut hilera).expect("Failed");
    println!("Hilera ingresada : '{}'.",hilera);

    let hilera:String=hilera.trim().parse().expect("Failed");
    println!("Hilera ingresada despues de trim().parse() : '{}'.",hilera);
 
}