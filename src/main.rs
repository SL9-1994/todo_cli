mod task;
use clap::{Args, Parser, Subcommand};
use std::env;
use task::Tasks;

/// The main CLI struct.
#[derive(Parser, Debug)]
#[command(author, version, about = "Simple todo CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// The available commands for the CLI.
#[derive(Subcommand, Debug)]
enum Commands {
    Add(AddArgs),
    Remove(RemoveArgs),
    Edit(EditArgs),
    List(ListArgs),
}

/// The arguments for the "add" command.
#[derive(Args, Debug)]
#[command(about = "Add todo task")]
struct AddArgs {
    #[arg(short, long)]
    title: String,

    #[arg(short, long)]
    description: String,
}

/// The arguments for the "edit" command.
#[derive(Args, Debug)]
#[command(about = "Edit todo task")]
struct EditArgs {
    #[arg()]
    id: String,

    #[arg(short, long)]
    title: Option<String>,

    #[arg(short, long)]
    description: Option<String>,

    #[arg(short, long)]
    is_done: Option<bool>,
}

/// The arguments for the "remove" command.
#[derive(Args, Debug)]
#[command(about = "Remove todo task")]
struct RemoveArgs {
    #[arg()]
    id: String,
}

/// The arguments for the "list" command.
#[derive(Args, Debug)]
#[command(about = "Show todo tasks")]
struct ListArgs;

fn main() {
    let args = Cli::parse();

    let todo_file = env::var("TODO_FILE").unwrap_or_else(|_| "/tmp/todo/todo.csv".to_string());

    let mut tasks = Tasks::new(todo_file);

    match args.command {
        Commands::Add(add_args) => {
            let result = tasks.add_task(add_args);
            match result {
                Ok(res) => println!("{}", res),
                Err(e) => eprintln!("Error adding task:  {}", e),
            }
        }

        Commands::Remove(remove_args) => {
            let result = tasks.remove_task(remove_args);
            match result {
                Ok(res) => println!("{}", res),
                Err(e) => eprintln!("Error removing task: {}", e),
            }
        }

        Commands::Edit(edit_args) => {
            let result = tasks.edit_task(edit_args);
            match result {
                Ok(res) => println!("{}", res),
                Err(e) => eprintln!("Error editing task: {}", e),
            }
        }

        Commands::List(_) => {
            let result = tasks.list_task();
            match result {
                Ok(_) => {}
                Err(e) => eprintln!("Error listing tasks: {}", e),
            }
        }
    }
}
