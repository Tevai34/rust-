//! Memory Manager CLI
//!
//! This program reads a command file, parses commands, and interacts with a memory manager.

use rust_template::file_parser::FileParser;
use rust_template::memory_manager::MemoryManager;
use std::env;

/// The main entry point of the program.
fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <command_file>", args[0]);
        return;
    }

    let file_path = &args[1];

    // Initialize the file parser
    let parser = FileParser::new(file_path);
    
    // Initialize the memory manager
    let mut memory_manager = MemoryManager::new();

    // Process parsed commands
    for command in parser.commands {
        match command.function.as_str() {
            "INSERT" => {
                if command.parameters.len() >= 2 {
                    if let Ok(size) = command.parameters[0].parse::<usize>() {
                        let data = command.parameters[1].clone();
                        let id = memory_manager.insert(size, data);
                        println!("Inserted block with ID: {}", id);
                    } else {
                        eprintln!("Invalid INSERT command: size must be a number");
                    }
                } else {
                    eprintln!("Invalid INSERT command format: INSERT [BLOCK_SIZE] [DATA]");
                }
            }
            "READ" => {
                if let Some(id) = command.parameters.get(0).and_then(|s| s.parse::<usize>().ok()) {
                    match memory_manager.read(id) {
                        Some(data) => println!("Data at {}: {}", id, data),
                        None => println!("Nothing at {}", id),
                    }
                } else {
                    eprintln!("Invalid READ command format: READ [ID]");
                }
            }
            "DELETE" => {
                if let Some(id) = command.parameters.get(0).and_then(|s| s.parse::<usize>().ok()) {
                    memory_manager.delete(id);
                    println!("Deleted block {}", id);
                } else {
                    eprintln!("Invalid DELETE command format: DELETE [ID]");
                }
            }
            "UPDATE" => {
                if command.parameters.len() >= 2 {
                    if let Ok(id) = command.parameters[0].parse::<usize>() {
                        let new_data = command.parameters[1].clone();
                        if memory_manager.update(id, new_data) {
                            println!("Updated block {}", id);
                        } else {
                            println!("Nothing at {}", id);
                        }
                    } else {
                        eprintln!("Invalid UPDATE command: ID must be a number");
                    }
                } else {
                    eprintln!("Invalid UPDATE command format: UPDATE [ID] [NEW_DATA]");
                }
            }
            "DUMP" => {
                memory_manager.dump();
            }
            _ => {
                eprintln!("Unknown command: {}", command.function);
            }
        }
    }
}
