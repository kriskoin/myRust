use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("El tipo de la variable es: {}", type_name::<T>());
}

fn main(){
    let an_32_integer:i32=5;
    let an_64_integer:i64=an_32_integer as i64 ;

    println!("an_64_integer: {}",an_64_integer);

    //another method

    let integer_32:i32 = 5;
    let integer_64:i64 = integer_32.into();
    println!("integer_64: {}",integer_64);

    let integer_64:i64 = (integer_32+8).into();
    println!("integer_64: {}",integer_64);

    let integer_64:i64 = integer_32 as i64 +76;
    println!("integer_64: {}",integer_64);

    let a:i32=10;
    print_type_of(&a);
    {
        let a:i64 = a as i64 + 10;
        print_type_of(&a);
        println!("this is a :{}",a);
    }
    print_type_of(&a);
    
    
}