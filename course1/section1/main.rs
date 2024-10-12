fn main(){
    let an_integer:i32=-5;
    println!("esto es un entero inmutable { }",an_integer);
    //an_integer = 60; //fail

    let mut an_mut_integer:i32=-5;
    println!("esto es un entero mutable before { }",an_mut_integer);

    an_mut_integer=-55;
    println!("esto es un entero mutable after { }",an_mut_integer);

    //string
    let a="this an string";
    println!("{}",a);

    let b=String::new();
    println!("before push_str {}",b);
    //  error[E0596]
    //b.push_str(" a value for string ");
    println!("after push_str {}",b);


    let c=String::new();
    //error[E0384]:
    //c = String::from("Hello i'm cris");
    println!("before push_str {}",c);
    
    let mut d=String::from("XXX co");
    d = String::from("Hello i'm cris");
    println!("before push_str {}",d);

    //string
    let str1=String::from("this another string");
    println!("{}",str1);

    let mut str2=String::from("this another string");
    str2.push_str(" with MUTABLE ");
    println!("{}",str2);


}