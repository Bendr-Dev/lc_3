use crate::memory::Memory;

// Initialization

#[test]
fn test_memory_init() {
    let memory = Memory::new();

    assert_eq!(memory.0.len(), u16::MAX as usize + 1);
}

#[test]
fn test_memory_index() {
    let memory = Memory::new();

    assert_eq!(memory.0[0], 0x0);
}
