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
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};

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

#[derive(Deserialize, Serialize, Debug)]
struct Todo {
    name: String,
    description: String,
}

fn main() {
    let args: Args = Args::parse();

    match &args.command {
        Commands::Add { title, description } => {
            println!("Adding Item To Todo");
            println!("Title: {} Description: {}", title, description);
            let todo = Todo {
                name: title.to_string(),
                description: description.to_string(),
            };
            save_to_file(&todo);
        }
        Commands::List => {
            println!("Printing Todo List");
            read_from_file("todo.json");
        }
        Commands::Close => {
            println!("Ending Session");
        }
    }
}

fn save_to_file(t: &Todo) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("todo.json")
        .unwrap();

    let json = serde_json::to_string(t).unwrap();
    let json_bytes = json.as_bytes();

    match file.write_all(json_bytes) {
        Ok(()) => println!("Save successful"),
        Err(e) => println!("Failed to process: {}", e),
    }
}

fn read_from_file(file_name: &str) {
    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);

    for (index, line) in reader.lines().into_iter().enumerate() {
        println!("{}: {}", index, line.unwrap());
    }
}
