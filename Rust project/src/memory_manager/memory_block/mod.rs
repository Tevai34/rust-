/// Trait defining common memory block behavior
pub(crate) trait MemoryBlock {
    /// Returns the start address of the block
    fn get_start(&self) -> usize;
    
    /// Returns the size of the block
    fn get_size(&self) -> usize;
}

/// Represents an allocated memory block
#[derive(Debug, Clone, Copy)]
pub struct AllocatedBlock {
    start: usize,
    size: usize,
    pub id: usize, // ID should remain public for external referencing
}

impl AllocatedBlock {
    /// Creates a new allocated block
    pub fn new(start: usize, size: usize, id: usize) -> Self {
        Self { start, size, id }
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

/// Represents a free memory block
#[derive(Debug, Clone, Copy)]
pub struct FreeBlock {
    start: usize,
    size: usize,
}

impl FreeBlock {
    /// Creates a new free block
    pub fn new(start: usize, size: usize) -> Self {
        Self { start, size }
    }
}

impl MemoryBlock for FreeBlock {
    fn get_start(&self) -> usize {
        self.start
    }

    fn get_size(&self) -> usize {
        self.size
    }
}


