use crate::memory::Memory;

#[test]
fn test_memory_init() {
    let memory: Memory = Memory::new();

    assert_eq!(memory.0.len(), 65_536);
    assert_eq!(memory.0[0], 0x0);
}

#[test]
fn test_memory_write() {
    let mut memory: Memory = Memory::new();

    memory.write(0x3000, 0xFFFF);

    assert_eq!(memory.0[0x3000], 0xFFFF);
}

#[test]
fn test_memory_read() {
    let mut memory: Memory = Memory::new();

    memory.0[0x3000] = 0xFFFF;

    assert_eq!(memory.read(0x3000), 0xFFFF);
}
