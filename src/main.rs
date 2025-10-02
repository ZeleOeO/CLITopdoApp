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
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

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
    Delete { delete_index: i32 },
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
        Commands::Delete { delete_index } => {
            delete_todo("todo.json", *delete_index);
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

    match writeln!(file, "{}", json) {
        Ok(()) => println!("Save successful"),
        Err(e) => println!("Failed to process: {}", e),
    }
}

fn read_from_file(file_name: &str) {
    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);

    for (index, line) in reader.lines().into_iter().enumerate() {
        println!("{}: {}", index + 1, line.unwrap());
    }
}

fn delete_todo(file_name: &str, delete_index: i32) {
    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);

    let lines: Vec<String> = reader
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            let line = line.ok()?;
            if (i as i32) + 1 != delete_index {
                Some(line)
            } else {
                None
            }
        })
        .collect();

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name)
        .unwrap();

    for line in lines {
        writeln!(file, "{}", line).unwrap();
    }

    println!("Deleted todo at line {} ", delete_index);
}
