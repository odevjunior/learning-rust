fn main() {
    multiples_conditions();
    inline_if();
}

fn multiples_conditions() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn inline_if() {
    fn main() {
        let condition = true;
    
        let number = if condition { 5 } else { 6 };
    
        println!("The value of number is: {number}");

        //lembre-se o retorno do if precisa ser do mesmo tipo, ou tipos compativeis
    }
}
