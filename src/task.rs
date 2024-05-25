use std::error::Error;
use std::path::Path;
use std::{fs, io};

use prettytable::{row, Cell, Row, Table};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{AddArgs, EditArgs, RemoveArgs};
use csv::{Reader, Writer};
use std::fs::{File, OpenOptions};

/// Represents a collection of tasks.
#[derive(Debug, Serialize, Deserialize)]
pub struct Tasks {
    tasks: Vec<Task>,
    file_path: String,
}

impl Tasks {
    /// Creates a new instance of `Tasks`.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the CSV file that stores the tasks.
    pub fn new(file_path: String) -> Tasks {
        Tasks {
            tasks: Vec::new(),
            file_path,
        }
    }

    /// Adds a new task to the collection.
    ///
    /// # Arguments
    ///
    /// * `add_args` - The arguments for adding a task.
    ///
    /// # Returns
    ///
    /// * `Result<String, Box<dyn Error>>` - A result indicating whether the task was successfully added or not.
    pub fn add_task(&mut self, add_args: AddArgs) -> Result<String, Box<dyn Error>> {
        self.read_tasks_from_csv()?;

        let new_id = self.generate_task_id();
        let is_done = false;

        self.tasks.push(Task::new(
            new_id,
            add_args.title,
            add_args.description,
            is_done,
        ));

        self.write_tasks_to_csv()?;

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
    /// * `Result<String, Box<dyn Error>>` - A result indicating whether the task was successfully edited or not.
    pub fn edit_task(&mut self, edit_args: EditArgs) -> Result<String, Box<dyn Error>> {
        self.read_tasks_from_csv()?;

        let id = edit_args.id;
        let mut found = false;

        for task in &mut self.tasks {
            if task.id == id {
                if let Some(title) = edit_args.title {
                    task.title = title;
                }
                if let Some(description) = edit_args.description {
                    task.description = description;
                }
                if let Some(is_done) = edit_args.is_done {
                    task.is_done = is_done;
                }
                found = true;
                break;
            }
        }

        if found {
            self.write_tasks_to_csv()?;
            Ok("The task was successfully edited.".to_string())
        } else {
            Err("Task not found.".into())
        }
    }

    /// Lists all the tasks in the collection.
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn Error>>` - A result indicating whether the tasks were successfully listed or not.
    pub fn list_task(&mut self) -> Result<(), Box<dyn Error>> {
        self.read_tasks_from_csv()?;

        let mut table = Table::new();

        // Add a header
        table.add_row(row!["Id", "Title", "Desc", "Is Done"]);

        // Add a row and cells
        for task in &self.tasks {
            table.add_row(Row::new(vec![
                Cell::new(&task.id).style_spec("ubFG"),
                Cell::new(&task.title).style_spec("bFG"),
                Cell::new(&task.description).style_spec("bFG"),
                Cell::new(if task.is_done { "Yes" } else { "No" }).style_spec("bFG"),
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
    /// * `Result<String, Box<dyn Error>>` - A result indicating whether the task was successfully removed or not.
    pub fn remove_task(&mut self, remove_args: RemoveArgs) -> Result<String, Box<dyn Error>> {
        self.read_tasks_from_csv()?;
        let initial_len = self.tasks.len();
        self.tasks.retain(|task| task.id != remove_args.id);

        let new_len = self.tasks.len();

        if initial_len == new_len {
            return Err("Task not found.".into());
        }

        self.write_tasks_to_csv()?;
        Ok("The task was successfully removed.".to_string())
    }

    /// Writes all tasks in the collection to a CSV file.
    ///
    /// This method will overwrite the existing file if it exists, or create a new file if it does not.
    /// Each task is serialized into a row in the CSV file.
    ///
    /// # Returns
    ///
    /// * `io::Result<()>` - A result indicating whether the tasks were successfully written to the CSV file.
    fn write_tasks_to_csv(&self) -> io::Result<()> {
        let mut writer = if Path::new(&self.file_path).exists() {
            let file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&self.file_path)?;
            Writer::from_writer(file)
        } else {
            if let Some(parent) = Path::new(&self.file_path).parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            let file = File::create(&self.file_path)?;
            Writer::from_writer(file)
        };

        for task in &self.tasks {
            writer.serialize(task)?;
        }

        writer.flush()?;
        Ok(())
    }

    /// Reads tasks from a CSV file.
    ///
    /// This method reads tasks from a CSV file and adds them to the internal task list.
    /// The CSV file should exist at the path specified when this `Tasks` instance was created.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the tasks were successfully read.
    /// * `Err(io::Error)` - If an error occurred while reading the tasks.
    fn read_tasks_from_csv(&mut self) -> io::Result<()> {
        self.tasks.clear();
        if Path::new(&self.file_path).exists() {
            let mut reader = Reader::from_path(&self.file_path)?;

            for result in reader.deserialize() {
                let task = result?;
                self.tasks.push(task);
            }
        }

        Ok(())
    }

    /// Generates a new task ID.
    ///
    /// This method is used to provide a unique ID for a new task.
    /// The generated ID consists of a random combination of alphabets and numbers.
    ///
    /// # Returns
    ///
    /// * `String` - The generated task ID.
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
