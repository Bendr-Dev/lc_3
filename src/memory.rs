use std::{
    io::Read,
    ops::{Index, IndexMut},
};

type AddressSpace = [u16; u16::MAX as usize + 1];

pub struct Memory(AddressSpace);

impl Memory {
    pub fn new() -> Self {
        Memory([0x0; u16::MAX as usize + 1])
    }

    pub fn read(&mut self, address: u16) -> u16 {
        match address {
            0xFE00 => {
                let mut buffer: [u8; 1] = [0 as u8; 1];
                std::io::stdin().read_exact(&mut buffer).unwrap();

                match buffer[0] {
                    0 => self.write(0xFE00, 0),
                    char_value => {
                        self.write(0xFE00, 1 << 15);
                        self.write(0xFE02, char_value as u16)
                    }
                }
            }
            _ => {}
        }

        return self.0[address as usize];
    }

    pub fn write(&mut self, address: u16, value: u16) {
        self.0[address as usize] = value;
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
