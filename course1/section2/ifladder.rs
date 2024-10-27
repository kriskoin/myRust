fn main (){
    let a = 100;
    let b = 2000;
    let c = 500;
    if a > b && a > c {
        println!("A is greatest");
    }else if b>a && b>c {
        println! ("B is greatest");
    }else{
        println!("C is greatest");
    }
}