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
    ///
    /// # Arguments
    /// * start - The starting index of the block
    /// * size - The size of the block
    /// * id - The identifier for the block
    ///
    /// # Returns
    /// * A new instance of AllocatedBlock
    pub fn new(start: usize, size: usize, id: usize) -> Self {
        Self {
            start,
            size,
            id,
            is_free: false, // Default to allocated
        }
    }

    /// Marks a block as free
    pub fn free(&mut self) {
        self.is_free = true;
    }

    /// Checks if a block can be merged with another
    ///
    /// # Arguments
    /// * other - The other block to check for merging
    ///
    /// # Returns
    /// * true if the blocks are adjacent and both free, otherwise false
    pub fn can_merge(&self, other: &AllocatedBlock) -> bool {
        self.is_free && other.is_free && (self.start + self.size == other.start)
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
