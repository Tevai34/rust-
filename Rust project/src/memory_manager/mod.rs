mod memory_block;
mod allocated_block;
mod free_block;

use allocated_block::AllocatedBlock;
use free_block::FreeBlock;

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
}
