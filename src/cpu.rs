use crate::memory::Memory;

pub struct CPU {
    registers: [u16; 8],
    program_counter: u16,
    processor_status_register: u16,
    memory: Memory,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0x0; 8],
            program_counter: 0x3000,
            memory: Memory::new(),
            processor_status_register: 0x2, // 2 sets the zero register flag to HIGH
        }
    }

    pub fn tick(&mut self) {
        let curr_op: u16 = self.memory[self.program_counter];
        self.program_counter += 1;
        let op_code: u8 = ((curr_op & 0xF000) >> 12) as u8;

        match op_code {
            // Instruction set

            // Operate instructions
            0x9 => self.not(curr_op),
            0x1 => self.add(curr_op),
            0x5 => self.and(curr_op),
            // Data Movement instructions
            0x2 => self.load(curr_op),
            0x3 => self.store(curr_op),
            0xA => self.load_indirect(curr_op),
            0xB => self.store_indirect(curr_op),
            0x6 => self.load_offset(curr_op),
            0x7 => self.store_offset(curr_op),
            0xE => self.load_immediate(curr_op),
            // Control instructions
            0x0 => unimplemented!("Branch instruction"),
            0xC => unimplemented!("Jump instruction"),
            0xF => unimplemented!("TRAP"),
            // Reserved instruction
            0xD => unimplemented!("Reserved operation"),
            _ => unreachable!("Bad op code"),
        }
    }

    fn not(&mut self, operation: u16) {
        let dst: u8 = ((operation & 0x0E00) >> 9) as u8;
        let src: u8 = ((operation & 0x0160) >> 6) as u8;

        self.registers[dst as usize] = !(self.registers[src as usize]);
        self.set_condition_codes(self.registers[dst as usize]);
    }

    fn add(&mut self, operation: u16) {
        let dst: u8 = ((operation & 0x0E00) >> 9) as u8;
        let src1: u8 = ((operation & 0x0160) >> 6) as u8;
        let register_mode: u8 = ((operation & 0x0020) >> 4) as u8;

        match register_mode {
            0x0 => {
                let src2: u8 = (operation & 0x0007) as u8;
                let result: u16 =
                    self.registers[src1 as usize].wrapping_add(self.registers[src2 as usize]);
                self.registers[dst as usize] = result;
                self.set_condition_codes(result);
            }
            0x2 => {
                let immediate: u16 = self.sign_extension(operation & 0x001F, 5);
                let result: u16 = self.registers[src1 as usize].wrapping_add(immediate);
                self.registers[dst as usize] = result;
                self.set_condition_codes(result);
            }
            _ => unreachable!("Invalid register mode."),
        }
    }

    fn and(&mut self, operation: u16) {
        let dst: u8 = ((operation & 0x0E00) >> 9) as u8;
        let src1: u8 = ((operation & 0x0160) >> 6) as u8;
        let register_mode: u8 = ((operation & 0x0020) >> 4) as u8;

        match register_mode {
            0x0 => {
                let src2: u8 = (operation & 0x0007) as u8;
                let result: u16 = self.registers[src1 as usize] & self.registers[src2 as usize];
                self.registers[dst as usize] = result;
                self.set_condition_codes(result);
            }
            0x2 => {
                let immediate: u16 = self.sign_extension(operation & 0x001F, 5);
                let result: u16 = self.registers[src1 as usize] & immediate;
                self.registers[dst as usize] = result;
                self.set_condition_codes(result);
            }
            _ => unreachable!("Invalid register mode."),
        }
    }

    fn load(&mut self, operation: u16) {
        let dst: u8 = ((operation & 0x0E00) >> 9) as u8;
        let signed_extension: u16 = self.sign_extension(operation & 0x01FF, 9);

        // TODO: test this, but probably going to have to make sure memory range is valid (0x3000 - 0xFDFF)
        let memory_address: u16 = self.program_counter.wrapping_add(signed_extension);

        self.registers[dst as usize] = self.memory[memory_address];
    }

    fn load_indirect(&mut self, operation: u16) {
        let dst: u8 = ((operation & 0x0E00) >> 9) as u8;
        let signed_extension: u16 = self.sign_extension(operation & 0x01FF, 9);

        let memory_address: u16 = self.program_counter.wrapping_add(signed_extension);

        self.registers[dst as usize] = self.memory[self.memory[memory_address]];
    }

    fn load_offset(&mut self, operation: u16) {
        let dst: u8 = ((operation & 0x0E00) >> 9) as u8;
        let base: u8 = ((operation & 0x01C0) >> 6) as u8;
        let offset: u16 = self.sign_extension(operation & 0x003F, 6);

        let memory_address: u16 = self.registers[base as usize].wrapping_add(offset);

        let result: u16 = self.memory[memory_address];
        self.registers[dst as usize] = result;
        self.set_condition_codes(result);
    }

    fn load_immediate(&mut self, operation: u16) {
        let dst: u8 = ((operation & 0x0E00) >> 9) as u8;
        let signed_extension: u16 = self.sign_extension(operation & 0x01FF, 9);

        let memory_address: u16 = self.program_counter.wrapping_add(signed_extension);

        self.registers[dst as usize] = memory_address;
    }

    fn store(&mut self, operation: u16) {
        let src: u8 = ((operation & 0x0E00) >> 9) as u8;
        let signed_extension: u16 = self.sign_extension(operation & 0x01FF, 9);

        let memory_address: u16 = self.program_counter.wrapping_add(signed_extension);

        self.memory[memory_address] = self.registers[src as usize];
    }

    fn store_indirect(&mut self, operation: u16) {
        let src: u8 = ((operation & 0x0E00) >> 9) as u8;
        let signed_extension: u16 = self.sign_extension(operation & 0x01FF, 9);

        let memory_address: u16 = self.program_counter.wrapping_add(signed_extension);
        let memory_address_content: u16 = self.memory[memory_address];

        self.memory[memory_address_content] = self.registers[src as usize];
    }

    fn store_offset(&mut self, operation: u16) {
        let src: u8 = ((operation & 0x0E00) >> 9) as u8;
        let base: u8 = ((operation & 0x01C0) >> 6) as u8;
        let offset: u16 = self.sign_extension(operation & 0x003F, 6);

        let memory_address: u16 = self.registers[base as usize].wrapping_add(offset);

        self.memory[memory_address] = self.registers[src as usize];
    }

    fn sign_extension(&self, mut bits: u16, bit_count: usize) -> u16 {
        if (bits >> (bit_count - 1)) & 1 == 1 {
            bits |= 0xFFFF << bit_count;
        }

        return bits;
    }

    fn set_condition_codes(&mut self, result: u16) {
        self.processor_status_register = self.processor_status_register & 0xFFF8;

        match result {
            x if ((x & 0x8000) >> 15) == 1 => self.processor_status_register |= 0b100,
            x if x == 0 => self.processor_status_register |= 0b010,
            _ => self.processor_status_register |= 0b001,
        };
    }
}

// Points to test file instead of directly testing here
#[cfg(test)]
#[path = "./cpu_test.rs"]
mod cpu_test;