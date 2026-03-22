use std::io; //get the toold from the folder std/io



fn main(){
    println!("likh na kuch bhadwa");

    let mut input = String::new(); // creates new empty bucket 

    io::stdin()
        .read_line(&mut input)
        .expect("Failed");

    println!("you wrote {}", input);
    
}