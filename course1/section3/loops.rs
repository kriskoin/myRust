fn main (){
    let mut n = 0;
    loop {
        if n<7 {
            println!("Hello {} \n",n);
            n+=1;
        }else{
            break;
        }
    }

    n = 0;

    while n <= 5 {
        println!("Hello 2 {}\n",n);
        n+=1;

    }

    n = 0 ;

    for n in 1 .. 10 {
        println!("Hello 3 {}\n",n);
    }
}