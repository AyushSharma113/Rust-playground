use std::io; //get the tool from the folder std/io
use rand::Rng;
use std::cmp::Ordering;



// this is how we import tools in rust

// : u32: This is a type annotation. 
// parse() is a general tool that can convert 
// strings into almost anything. By adding : u32 (Unsigned 32-bit integer—meaning a positive whole number),
//  you are telling .parse() exactly what to turn the text into.



// trim() = removes extra spaces and invisible enter keys values
// parse() = convert the text into a actual number

fn main(){

    let secret = rand::thread_rng().gen_range(1..=10);
    loop {
        println!("Guess kar bhadwa:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("kuch ho giya re baba");

        let guess: u32 = guess.trim().parse().expect("lawra phir garbar ho gya");

        match guess.cmp(&secret) {
            Ordering::Less => println!("number nunu"),
            Ordering::Greater => println!("your dick size"),
            Ordering::Equal => {
                println!("khus mt ho , jit ke kon sa jhat ka bal ukhad lega");
                break;
            }
        }
        
        
    }
}