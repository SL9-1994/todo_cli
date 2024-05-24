use std::error::Error;
use std::io::Write;
use std::path::Path;
use std::{fs, io};

use prettytable::{row, Cell, Row, Table};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{AddArgs, EditArgs, RemoveArgs};
use csv::{Reader, Writer, WriterBuilder};
use std::fs::{File, OpenOptions};

// Constants

/// The file path of the todo list CSV file.
/// Consider reading from environment variables.
const TODO_FILE: &str = "/tmp/todo/todo.csv";

// end Constants

/// Represents a collection of tasks.
#[derive(Debug, Serialize, Deserialize)]
pub struct Tasks {
    tasks: Vec<Task>,
}

impl Tasks {
    /// Creates a new instance of `Tasks`.
    pub fn new() -> Tasks {
        Tasks { tasks: Vec::new() }
    }

    /// Adds a new task to the collection.
    ///
    /// # Arguments
    ///
    /// * `add_args` - The arguments for adding a task.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating whether the task was successfully added or not.
    pub fn add_task(&mut self, add_args: AddArgs) -> Result<String, Box<dyn Error>> {
        let new_id = self.generate_task_id();
        let is_done = false; // Initial value

        self.tasks.push(Task::new(
            new_id,
            add_args.title,
            add_args.description,
            is_done,
        ));

        self.write_task_to_csv()?;

        Ok("The task was successfully added.".to_string())
    }

    /// Edits an existing task in the collection.
    ///
    /// # Arguments
    ///
    /// * `edit_args` - The arguments for editing a task.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating whether the task was successfully edited or not.
    pub fn edit_task(&mut self, edit_args: EditArgs) -> Result<String, Box<dyn Error>> {
        let id = edit_args.id;

        let file = File::open(TODO_FILE)?;

        let mut rdr = csv::Reader::from_reader(file);

        // Since it is not possible to read and write files at the same time, prepare a buffer and
        //Edit the task and write the edited task to the buffer
        let mut buffer = Vec::new();
        {
            let mut writer = csv::Writer::from_writer(&mut buffer);

            for result in rdr.deserialize() {
                let mut record: Task = result?;
                if record.id == id {
                    let title = edit_args.title.clone().unwrap_or(record.title);
                    let description = edit_args.description.clone().unwrap_or(record.description);
                    let is_done = edit_args.is_done.unwrap_or(record.is_done);

                    record.title = title;
                    record.description = description;
                    record.is_done = is_done;
                }
                writer.serialize(&record)?;
            }
        }

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(TODO_FILE)?;
        file.write_all(&buffer)?;

        Ok("The task was successfully edited.".to_string())
    }

    /// Lists all tasks in the collection.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating whether the tasks were successfully listed or not.
    pub fn list_task(&mut self) -> Result<(), Box<dyn Error>> {
        self.read_task_to_csv()?;

        let mut table = Table::new();

        // Add a header
        table.add_row(row!["id", "title", "desc", "is_done"]);

        // Add a row and cells
        for task in &self.tasks {
            table.add_row(Row::new(vec![
                Cell::new(&task.id).style_spec("ubFG"),
                Cell::new(&task.title).style_spec("bFG"),
                Cell::new(&task.description).style_spec("bFG"),
                Cell::new(if task.is_done { "〇" } else { "×" }).style_spec("bFG"),
            ]));
        }

        table.printstd();

        Ok(())
    }

    /// Removes a task from the collection.
    ///
    /// # Arguments
    ///
    /// * `remove_args` - The arguments for removing a task.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating whether the task was successfully removed or not.
    pub fn remove_task(&mut self, remove_args: RemoveArgs) -> Result<String, Box<dyn Error>> {
        let file = File::open(TODO_FILE)?;
        let mut rdr = csv::Reader::from_reader(file);
        let mut buffer = Vec::new();

        {
            let mut writer = csv::Writer::from_writer(&mut buffer);
            for result in rdr.deserialize() {
                let record: Task = result?;
                if record.id != remove_args.id {
                    writer.serialize(&record)?;
                }
            }
        }

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(TODO_FILE)?;
        file.write_all(&buffer)?;

        Ok("The task was successfully removed.".to_string())
    }

    fn write_task_to_csv(&self) -> io::Result<()> {
        let mut writer = if Path::new(TODO_FILE).exists() {
            let file = OpenOptions::new().append(true).open(TODO_FILE)?;
            // Ignore the header
            WriterBuilder::new().has_headers(false).from_writer(file)
        } else {
            // Create the directory if it does not exist
            // Ensure that the parent directory of TODO_FILE exists
            if let Some(parent) = Path::new(TODO_FILE).parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }

            // Write the header
            let file = File::create(TODO_FILE)?;
            Writer::from_writer(file)
        };

        for task in &self.tasks {
            writer.serialize(task)?;
        }

        writer.flush()?;

        Ok(())
    }

    fn read_task_to_csv(&mut self) -> io::Result<()> {
        let mut reader = Reader::from_path(TODO_FILE)?;

        for result in reader.deserialize() {
            let task = result?;
            self.tasks.push(task);
        }

        Ok(())
    }

    fn generate_task_id(&self) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect()
    }
}

/// Represents a task.
#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: String,
    title: String,
    description: String,
    is_done: bool,
}

impl Task {
    /// Creates a new instance of `Task`.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the task.
    /// * `title` - The title of the task.
    /// * `description` - The description of the task.
    /// * `is_done` - Indicates whether the task is done or not.
    fn new(id: String, title: String, description: String, is_done: bool) -> Task {
        Task {
            id,
            title,
            description,
            is_done,
        }
    }
}
