fn main () {
    let num = if 1>20 {
        println!("if block");
        1
    } else {
        println!("if block");
        2
    };

    println!(" num value ={}",num);


    let num2 = if 1>20 {
        println!("if block");
        ()
    } else {
        println!("if block");
        ()
    };

    println!(" num2 value ={:?}",num2);
}