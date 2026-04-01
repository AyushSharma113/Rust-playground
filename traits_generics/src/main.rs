trait Speak {
    fn speak(&self);
}


struct Dog;
struct Cat;

impl Speak for Dog{
    fn speak(&self){
        println!("bho bho bhosdike");
    }
}

impl Speak for Cat {
    fn speak(&self){
        println!("meow meow ");
    }
}


