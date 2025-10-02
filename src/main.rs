//NOTE
//Stuff I want to do
//So I want to initalize it, when I initalize it will create a new todo list. It will also create a
//boolean to show it's been initialized
//Then when I'm adding to the todo list, I might use a hashmap, that will automatically generate a
//key or something when it's been created for easy reference to it
//Persistence is important
//And then add a nice cute table

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "todo_app")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { title: String, description: String },
    List,
    Close,
}

#[derive(Deserialize, Debug)]
struct Todo {
    name: String,
    description: String,
}

fn add_to_list(list: &mut HashMap<i32, Todo>, todo: Todo) {
    let last = list.keys().cloned().max().unwrap_or(0);
    list.insert(last + 1, todo);
}

fn main() {
    let args: Args = Args::parse();
    let mut todo_list: HashMap<i32, Todo> = HashMap::new();

    match &args.command {
        Commands::Add { title, description } => {
            println!("Adding Item To Todo");
            println!("Title: {} Description: {}", title, description);
            let todo = Todo {
                name: title.to_string(),
                description: description.to_string(),
            };
            add_to_list(&mut todo_list, todo);
        }
        Commands::List => {
            println!("Printing Todo List");
            println!("{:?}", todo_list);
        }
        Commands::Close => {
            println!("Ending Session");
        }
    }
}
