use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

pub mod command;

/// Parses command files and extracts commands.
#[derive(Debug)]
pub struct FileParser {
    pub commands: Vec<command::Command>,
}

impl FileParser {
    /// Creates a new `FileParser` instance from a file.
    ///
    /// # Arguments
    /// * `file_path` - A string slice representing the file path.
    ///
    /// # Returns
    /// * `io::Result<FileParser>` - A `Result` containing the `FileParser` instance if successful.
    pub fn new(file_path: &str) -> io::Result<Self> {
        let commands = Self::parse_commands(file_path)?;
        Ok(Self { commands })
    }

    /// Reads and parses commands from the given file.
    fn parse_commands(file_path: &str) -> io::Result<Vec<command::Command>> {
        let mut commands = Vec::new();

        let lines = Self::read_lines(file_path)?;
        for line in lines.flatten() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if !parts.is_empty() {
                let function = parts[0].to_string();
                let parameters = parts[1..].iter().map(|s| s.to_string()).collect();
                commands.push(command::Command::new(function, parameters));
            }
        }

        Ok(commands)
    }

    /// Reads lines from a file and returns an iterator.
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where
        P: AsRef<Path>,
    {
        let file = fs::File::open(&filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
