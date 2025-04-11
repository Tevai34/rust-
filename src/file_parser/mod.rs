use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

pub mod command;

#[derive(Debug)]
pub struct FileParser {
    pub commands: Vec<command::Command>,
}

impl FileParser {
    pub fn new(file_path: &str) -> io::Result<Self> {
        let commands = Self::parse_commands(file_path)?;
        Ok(Self { commands }) 
    }

    fn parse_commands(file_path: &str) -> io::Result<Vec<command::Command>> {
        let mut commands = Vec::new();

        let lines = Self::read_lines(file_path)?;
        for (i, line) in lines.flatten().enumerate() {
            let clean_line = line.trim().strip_suffix(';').unwrap_or(&line).trim(); 
            println!("Parsing line {}: {:?}", i + 1, clean_line);  
        
            let parts: Vec<&str> = clean_line.split_whitespace().collect();
            if !parts.is_empty() {
                let function = parts[0].to_string();
                let parameters = parts[1..].iter().map(|s| s.to_string()).collect();
                commands.push(command::Command::new(function, parameters));
            }
        }
        

        Ok(commands)
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where
        P: AsRef<Path>,
    {
        let file = fs::File::open(&filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
