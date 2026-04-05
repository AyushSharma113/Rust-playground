// derive = macro 
// Debug = allows you to print the struct using the "programmer's format{:?}""
// Clone = adds .clone() method to your struct.

// usize = unsigned integer


#[derive(Debug, Clone)]
pub struct Note {
    pub id: usize,
    pub content: String
}