use serde::{Serialize, Deserialize};

// derive is a code generator. it tells the compiler to autommatically 
// write the implementation for certain "Traits" (interfaces)
// so you dont have to do it manually.

// Serialize - converts the code to json
//  deserialize - converts the code to json to struct

#[derive(Serialize, Deserialize , Debug)]  // derive = procedural macro - auto implementation genrator for traits
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}





