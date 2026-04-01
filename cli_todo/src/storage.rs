///conerts the vector into a formatted (pretty-printed) json string.
/// unwrap() is used ere assuming the data is valid and serializable
use std::fs;
use std::io::Write;
use crate::todo::Todo; // crate refers to the root of the project


const FILE_PATH: &str = "todos.json";


pub fn load_todos() -> Vec<Todo>{
    let data = fs::read_to_string(FILE_PATH);

    match data {
        Ok(content) => serde_json::from_str(&content).unwrap_or(vec![]),
        Err(_) => vec![],
    }
}
// .unwrap_or() method call. You are passing the empty vector vec![] as an argument to that method. It’s saying: "Give me the parsed data, or if that's not possible, use this empty list as the fallback."



pub fn save_todos(todos: &Vec<Todo>){
    let json = serde_json::to_string_pretty(todos).unwrap(); // Result<String, Error>
    
    let mut file = fs::File::create(FILE_PATH).unwrap();
    
    //  converts the json string into raw bytes and writes them to the disk
    file.write_all(json.as_bytes()).unwrap(); 
}


