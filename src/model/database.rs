#[derive(Debug)]
pub struct Database {
    file_path: String,
}

impl Database {
    /// create a Database instance from file path
    pub fn new(file_path: &str) -> Database {
        println!("Creating Database from {file_path}");

        // using to_owned is preferred over to_string
        Database {
            file_path: file_path.to_owned(),
        }
    }
}
