use std::ops::{Index, IndexMut};

type AddressSpace = [u16; u16::MAX as usize + 1];

pub struct Memory(AddressSpace);

impl Memory {
    pub fn new() -> Self {
        Memory([0x0; u16::MAX as usize + 1])
    }
}

impl Index<u16> for Memory {
    type Output = u16;

    fn index(&self, i: u16) -> &Self::Output {
        &self.0[i as usize]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, i: u16) -> &mut Self::Output {
        &mut self.0[i as usize]
    }
}

#[cfg(test)]
#[path = "./memory_test.rs"]
mod memory_test;
