use super::MemoryBlock;

/// Represents an allocated memory block
pub struct AllocatedBlock {
    pub start: usize,
    pub size: usize,
    pub id: usize,
    pub is_free: bool,
}

impl AllocatedBlock {
    /// Creates a new allocated block
    pub fn new(start: usize, size: usize, id: usize) -> Self {
        Self {
            start,
            size,
            id,
            is_free: false,
        }
    }

    /// Marks a block as free
    pub fn free(&mut self) {
        self.is_free = true;
    }

    /// Checks if a block can be merged with another
    pub fn can_merge(&self, other: &AllocatedBlock) -> bool {
        self.is_free && other.is_free && (self.start + self.size == other.start)
    }

    pub fn split(&self, requested_size: usize) -> Option<AllocatedBlock> {
        if requested_size >= self.size {
            None
        } else {
            Some(AllocatedBlock::new(
                self.start + requested_size,
                self.size - requested_size,
                self.id + 1, 
            ))
        }
    }
}

impl MemoryBlock for AllocatedBlock {
    fn get_start(&self) -> usize {
        self.start
    }

    fn get_size(&self) -> usize {
        self.size
    }
}
