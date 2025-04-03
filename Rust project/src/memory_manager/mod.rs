mod memory_block;

use memory_block::allocated_block::AllocatedBlock;
use memory_block::free_block::FreeBlock;

pub struct MemoryManager {
    data: Vec<u8>,
    free_handles: Vec<FreeBlock>,
    allocated_handles: Vec<AllocatedBlock>,
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            data: vec![0; 65535], /// Initializes a 65535-byte memory array
            free_handles: vec![FreeBlock { start: 0, size: 65535 }], /// Initially one big free block
            allocated_handles: Vec::new(),
        }
    }

    pub fn insert(&mut self, size: usize, data: String) -> usize{
        0
    }
    pub fn read(&self, id: usize) -> Option<&str> {
        // Read logic
        Some("example")
    }
    pub fn delete(&mut self, id: usize) {
        // Delete logic
    }
    pub fn update(&mut self, id: usize, new_data: String) -> bool {
        // Update logic
        true
    }
    pub fn dump(&self) {
        // Dump memory state
    }
    
}

