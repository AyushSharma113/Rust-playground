mod todo;
mod storage;
mod cli;

use todo::Todo;
use storage::{load_todos, save_todos};
use cli::{Cli, Commands};
use clap::Parser;


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { title } => {
            let mut todos = load_todos();

            let id = todos.last().map_or(1, |t| t.id + 1);
            let todo = Todo {id, title, completed: false};

            todos.push(todo);
            save_todos(&todos);

            println!("Todo added");
        }

        Commands::List => {
            let todos = load_todos();

            if todos.is_empty(){
                println!("no todos found");
                return;
            }

            for todo in todos {
                let status = if todo.completed {"[X]"} else {"[ ]"};
                println!("{},{},{}", todo.id, status, todo.title);

            } 
        }

        Commands::Done { id } => {
            let mut todos = load_todos();

            for todo in &mut todos {
                if todo.id == id {
                    todo.completed = true;
                }
            }

            save_todos(&todos);
            println!("Todo marked as done");
            
        }
        
        Commands::Delete { id } => {
            let mut todos = load_todos();

            todos.retain(|t| t.id != id);

            save_todos(&todos);
            println!("todo deleted!");
        }
        
    }
    
}


