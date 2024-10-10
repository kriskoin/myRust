fn main(){
    let an_integer:i32=-5;
    println!("esto es un entero inmutable { }",an_integer);
    //an_integer = 60; //fail

    let mut an_mut_integer:i32=-5;
    println!("esto es un entero mutable before { }",an_mut_integer);

    an_mut_integer=-55;
    println!("esto es un entero mutable after { }",an_mut_integer);

    //string
    let str1=String::from("this an string");
    println!("{}",str1);

    let mut str2=String::from("this another string");
    str2.push_str(" with MUTABLE ");
    println!("{}",str2);


}