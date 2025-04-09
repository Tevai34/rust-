#[cfg(test)]
mod tests {
    use super::memory_manager::MemoryManager;

    // -------- INSERT TESTS --------
    #[test]
    fn test_insert_and_read() {
        let mut manager = MemoryManager::new();
        let id = manager.insert(8, "hello".to_string());
        let result = manager.read(id);
        assert_eq!(result, Some("hello".to_string()));
    }

    #[test]
    fn test_multiple_inserts() {
        let mut manager = MemoryManager::new();
        let id1 = manager.insert(8, "first".to_string());
        let id2 = manager.insert(8, "second".to_string());
        assert_ne!(id1, id2);
        assert_eq!(manager.read(id1), Some("first".to_string()));
        assert_eq!(manager.read(id2), Some("second".to_string()));
    }

    #[test]
    fn test_insert_and_read_empty_string() {
        let mut manager = MemoryManager::new();
        let id = manager.insert(4, "".to_string());
        assert_eq!(manager.read(id), Some("".to_string()));
    }

    #[test]
    fn test_insert_zero_size() {
        let mut manager = MemoryManager::new();
        let id = manager.insert(0, "zero".to_string());
        assert_eq!(manager.read(id), Some("zero".to_string()));
    }

    #[test]
    fn test_insert_with_large_size() {
        let mut manager = MemoryManager::new();
        let id = manager.insert(1024, "big".to_string());
        assert_eq!(manager.read(id), Some("big".to_string()));
    }

    // -------- UPDATE TESTS --------
    #[test]
    fn test_update() {
        let mut manager = MemoryManager::new();
        let id = manager.insert(8, "data".to_string());
        let updated = manager.update(id, "new".to_string());
        assert!(updated);
        assert_eq!(manager.read(id), Some("new".to_string()));
    }

    #[test]
    fn test_update_invalid_id() {
        let mut manager = MemoryManager::new();
        let updated = manager.update(999, "nothing".to_string());
        assert!(!updated);
    }

    #[test]
    fn test_update_to_empty_string() {
        let mut manager = MemoryManager::new();
        let id = manager.insert(4, "old".to_string());
        manager.update(id, "".to_string());
        assert_eq!(manager.read(id), Some("".to_string()));
    }

    #[test]
    fn test_update_with_same_data() {
        let mut manager = MemoryManager::new();
        let id = manager.insert(4, "unchanged".to_string());
        let updated = manager.update(id, "unchanged".to_string());
        assert!(updated);
        assert_eq!(manager.read(id), Some("unchanged".to_string()));
    }

    #[test]
    fn test_update_after_delete() {
        let mut manager = MemoryManager::new();
        let id = manager.insert(8, "data".to_string());
        manager.delete(id);
        let updated = manager.update(id, "fail".to_string());
        assert!(!updated);
    }

    // -------- READ (GET) TESTS --------
    #[test]
    fn test_read_nonexistent_id() {
        let manager = MemoryManager::new();
        assert_eq!(manager.read(1234), None);
    }

    #[test]
    fn test_read_after_multiple_operations() {
        let mut manager = MemoryManager::new();
        let id = manager.insert(8, "hi".to_string());
        manager.update(id, "yo".to_string());
        assert_eq!(manager.read(id), Some("yo".to_string()));
        manager.delete(id);
        assert_eq!(manager.read(id), None);
    }

    #[test]
    fn test_read_after_update() {
        let mut manager = MemoryManager::new();
        let id = manager.insert(10, "start".to_string());
        manager.update(id, "finish".to_string());
        assert_eq!(manager.read(id), Some("finish".to_string()));
    }

    #[test]
    fn test_read_after_delete_and_reinsert() {
        let mut manager = MemoryManager::new();
        let id1 = manager.insert(8, "first".to_string());
        manager.delete(id1);
        let id2 = manager.insert(8, "second".to_string());
        assert_ne!(id1, id2);
        assert_eq!(manager.read(id2), Some("second".to_string()));
    }

    #[test]
    fn test_read_empty_memory() {
        let manager = MemoryManager::new();
        assert_eq!(manager.read(0), None);
    }

    // -------- DELETE TESTS --------
    #[test]
    fn test_delete() {
        let mut manager = MemoryManager::new();
        let id = manager.insert(8, "delete".to_string());
        manager.delete(id);
        assert_eq!(manager.read(id), None);
    }

    #[test]
    fn test_delete_twice() {
        let mut manager = MemoryManager::new();
        let id = manager.insert(8, "x".to_string());
        manager.delete(id);
        manager.delete(id); // Should not panic or error
        assert_eq!(manager.read(id), None);
    }

    #[test]
    fn test_delete_invalid_id() {
        let mut manager = MemoryManager::new();
        manager.delete(9876); // Should not panic
    }

    #[test]
    fn test_delete_after_update() {
        let mut manager = MemoryManager::new();
        let id = manager.insert(8, "x".to_string());
        manager.update(id, "y".to_string());
        manager.delete(id);
        assert_eq!(manager.read(id), None);
    }

    #[test]
    fn test_delete_does_not_affect_others() {
        let mut manager = MemoryManager::new();
        let id1 = manager.insert(8, "keep".to_string());
        let id2 = manager.insert(8, "remove".to_string());
        manager.delete(id2);
        assert_eq!(manager.read(id1), Some("keep".to_string()));
        assert_eq!(manager.read(id2), None);
    }
}
