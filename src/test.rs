#[cfg(test)]
mod tests {
    use crate::memory_manager::MemoryManager;
    use crate::file_parser::FileParser;
    #[test]
    fn test_insert_and_read() {
        let mut mm = MemoryManager::new();
        let id = mm.insert(10, String::from("hello"));
        assert_eq!(mm.read(id), Some("hello".to_string()));
    }

    #[test]
    fn test_update() {
        let mut mm = MemoryManager::new();
        let id = mm.insert(10, String::from("hello"));
        assert!(mm.update(id, String::from("world")));
        assert_eq!(mm.read(id), Some("world".to_string()));
    }

    #[test]
    fn test_delete() {
        let mut mm = MemoryManager::new();
        let id = mm.insert(10, String::from("test"));
        mm.delete(id);
        assert_eq!(mm.read(id), None);
    }

    #[test]
    fn test_reuse_freed_memory() {
        let mut mm = MemoryManager::new();
        let id1 = mm.insert(10, String::from("block1"));
        mm.delete(id1);
        let id2 = mm.insert(10, String::from("block2"));
        assert_eq!(mm.read(id2), Some("block2".to_string()));
    }
}
