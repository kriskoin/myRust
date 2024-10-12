fn main(){
    let a =10;
    let b = 7;
    println!("{}",a+b);
    println!("{}",2+2);
    println!("7 / 3 : {}",7/3);
    println!("15 % 2 : {}",15%2);
    println!("15.7 % 2.5 : {}",15.7%2.5);
    println!("15 / 2 : {}",15/2);
    println!("8 % 2 : {}",8%2);
    println!("8 / 2 : {}",8/2);
    println!("8 / 2 : {}",8/2);
    println!("8.13 < 2.5 : {}",8.13 < 2.5);
    println!("8.13 > 2.1 : {}",8.13 > 2.1);

    let _an_32_integer:i32=5;
    let _an_64_integer:i64=37;
    let _an_32_float:f32=1.5;
    let an_64_float:f64=31.5;
    let another_64_float:f64=11.5;

     /*
    println!("an_32_integer: {} > an_64_integer : {}  {}",an_32_integer,an_64_integer , an_32_integer > an_64_integer); //error
    println!("an_32_integer: {} > an_32_float : {}  {}",an_32_integer,an_32_float , an_32_integer > an_32_float); //error
    */

    println!("an_64_float: {} > another_64_float : {}  {}",an_64_float,another_64_float , an_64_float > another_64_float); //error

}