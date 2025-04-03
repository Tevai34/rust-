use super::MemoryBlock;

pub struct FreeBlock {
    pub start: usize,
    pub size: usize,
}

impl FreeBlock {
    /// Creates a new FreeBlock
    pub fn new(start: usize, size: usize) -> Self {
        FreeBlock { start, size }
    }

    /// Splits the free block into two if the requested size is smaller than the current size.
    /// Returns `Some(FreeBlock)` with the remaining space if a split occurs, otherwise `None`.
    pub fn split(&mut self, requested_size: usize) -> Option<FreeBlock> {
        if requested_size > self.size {
            return None;
        }
        let remaining_size = self.size - requested_size;
        self.size = requested_size; // Shrink the current block
        if remaining_size > 0 {
            Some(FreeBlock::new(self.start + requested_size, remaining_size))
        } else {
            None
        }
    }

    /// Checks if this block can merge with another free block (must be adjacent).
    pub fn can_merge(&self, other: &FreeBlock) -> bool {
        self.start + self.size == other.start || other.start + other.size == self.start
    }

    /// Merges this block with another adjacent free block and returns a new, larger block.
    pub fn merge(&self, other: &FreeBlock) -> FreeBlock {
        let new_start = usize::min(self.start, other.start);
        let new_size = self.size + other.size;
        FreeBlock::new(new_start, new_size)
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