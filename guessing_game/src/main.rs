use rand::Rng;
use std::io;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //pegando um numero aleatorio
    loop {
        let mut guess = String::new();
        
        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        //buscando a resposta do usuario

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //match eh como se fosse algum tipo de comparador, quase um switch-case

        println!("Yout guessed: {guess}");
        println!("the number is: {secret_number}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), //ordering eh uma forma de comparar um numero olhando se eh menor, maior, ou igual
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }    
}

/*
    obs: em rust todas as variaveis sao imutaveis, porem se adicionarmos a palavra reservada mut elas ficam mutaveis
    obs: o '&' significa que estou linkando uma variavel ja existente
 */