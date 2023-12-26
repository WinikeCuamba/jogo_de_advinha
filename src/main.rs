extern  crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Advinhe o numero!");
    let numero_secreto = rand::thread_rng().gen_range(1..101);


    loop {
        println!("Digite o seu palpapite");
        let mut palpite = String::new();

        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler entrada");

        let palpite: u32 = match palpite.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,

            };

        println!("Voce disse: {}", palpite);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo"),
            Ordering::Equal => {
                println!("Voce acertou");
                break
            },
            Ordering::Greater => println!("Muito alto"),
            
        }
    }
}
