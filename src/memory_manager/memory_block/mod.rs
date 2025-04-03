pub(crate) mod free_block;
pub(crate) mod allocated_block;

/// Trait defining common memory block behavior
pub(crate) trait MemoryBlock {
    /// Returns the start address of the block
    fn get_start(&self) -> usize;
    
    /// Returns the size of the block
    fn get_size(&self) -> usize;
}

