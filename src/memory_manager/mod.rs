mod memory_block;

use memory_block::allocated_block::AllocatedBlock;
use memory_block::free_block::FreeBlock;
use memory_block::MemoryBlock;

pub struct MemoryManager {
    data: Vec<u8>,
    free_handles: Vec<FreeBlock>,
    allocated_handles: Vec<AllocatedBlock>,
    next_id: usize,
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            data: vec![0; 65535],
            free_handles: vec![FreeBlock::new(0, 65535)],
            allocated_handles: Vec::new(),
            next_id: 1,
        }
    }

    pub fn insert(&mut self, size: usize, data: String) -> usize {
        let bytes = data.as_bytes();
        if bytes.len() > size {
            panic!("Data is larger than the block size.");
        }

        if let Some((index, _)) = self
            .free_handles
            .iter()
            .enumerate()
            .find(|(_, block)| block.size >= size)
        {
            let mut selected_block = self.free_handles.remove(index);
            let start = selected_block.start;

            // Split the free block
            if let Some(remaining_block) = selected_block.split(size) {
                self.free_handles.push(remaining_block);
            }

            // Write data
            for i in 0..bytes.len() {
                self.data[start + i] = bytes[i];
            }

            let block = AllocatedBlock::new(start, size, self.next_id);
            self.allocated_handles.push(block);

            self.next_id += 1;
            self.next_id - 1
        } else {
            panic!("No sufficient memory block found.");
        }
    }

    pub fn read(&self, id: usize) -> Option<String> {
        self.allocated_handles
            .iter()
            .find(|b| b.id == id && !b.is_free)
            .map(|block| {
                let slice = &self.data[block.start..block.start + block.size];
                let trimmed = slice.iter().take_while(|&&b| b != 0).cloned().collect::<Vec<u8>>();
                String::from_utf8(trimmed).unwrap_or_default()
            })
    }

    pub fn delete(&mut self, id: usize) {
        if let Some(index) = self.allocated_handles.iter().position(|b| b.id == id && !b.is_free) {
            let block = &mut self.allocated_handles[index];
            block.free();

            for i in block.start..block.start + block.size {
                self.data[i] = 0;
            }

            self.free_handles.push(FreeBlock::new(block.start, block.size));
        }
    }

    pub fn update(&mut self, id: usize, new_data: String) -> bool {
        if let Some(block) = self.allocated_handles.iter_mut().find(|b| b.id == id && !b.is_free) {
            let bytes = new_data.as_bytes();
            if bytes.len() > block.size {
                return false;
            }

            for i in 0..block.size {
                self.data[block.start + i] = if i < bytes.len() { bytes[i] } else { 0 };
            }

            true
        } else {
            false
        }
    }

    pub fn dump(&self) {
        println!("== Memory Dump ==");

        for block in &self.allocated_handles {
            let status = if block.is_free { "Free (was allocated)" } else { "Allocated" };
            let data_str = if !block.is_free {
                self.read(block.id).unwrap_or_default()
            } else {
                String::new()
            };
            println!(
                "Block ID {}: start={}, size={}, status={}, data='{}'",
                block.id,
                block.start,
                block.size,
                status,
                data_str
            );
        }

        for (i, block) in self.free_handles.iter().enumerate() {
            println!(
                "Free Block {}: start={}, size={}",
                i + 1,
                block.start,
                block.size
            );
        }
    }
}


