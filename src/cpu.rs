use std::{fs::File, io::Read, slice::Chunks};

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
            processor_status_register: 0x0,
        }
    }

    pub fn execute_program(&mut self, file_path: &String) {
        self.read_image(file_path);

        while self.program_counter < u16::MAX {
            self.tick();
        }
    }

    fn read_image(&mut self, file_path: &String) {
        // Attempt to read file path
        let mut file: File = match File::open(file_path) {
            Ok(ok_file) => ok_file,
            Err(_) => panic!("Error trying to read file with path: {}", file_path),
        };

        let mut data: Vec<u8> = Vec::new();

        file.read_to_end(&mut data).unwrap();

        match data.len() % 2 {
            result if result != 0 => panic!("Buffer size not even"),
            _ => {}
        }

        // Collect the data into chunks of size two 8 bit values as the lc3 stores data by 16 bits
        let mut data_chunks: Chunks<u8> = data.chunks(2);

        let program_counter_chunk = data_chunks.next().unwrap();

        let mut program_counter: u16 =
            u16::from_be_bytes([program_counter_chunk[0], program_counter_chunk[1]]);

        self.program_counter = program_counter; // Set program counter to origin provided by image (usually 0x3000)

        // Iterate through the rest of the chunks and insert into memory sequentially
        for data_chunk in data_chunks {
            self.memory.write(
                program_counter,
                u16::from_be_bytes([data_chunk[0], data_chunk[1]]),
            );
            program_counter = program_counter.wrapping_add(1);
        }
    }

    fn tick(&mut self) {
        let curr_op: u16 = self.memory.read(self.program_counter);
        self.program_counter = self.program_counter.wrapping_add(1);
        let op_code = curr_op >> 12;

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
            0x0 => self.branch(curr_op),
            0xC => self.jump(curr_op),
            0x4 => self.jump_register(curr_op),
            0xF => self.trap(curr_op),
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

        let memory_address: u16 = self.program_counter.wrapping_add(signed_extension);
        let result: u16 = self.memory.read(memory_address);

        self.registers[dst as usize] = result;
        self.set_condition_codes(result);
    }

    fn load_indirect(&mut self, operation: u16) {
        let dst: u8 = ((operation & 0x0E00) >> 9) as u8;
        let signed_extension: u16 = self.sign_extension(operation & 0x01FF, 9);

        let indirect_memory_address: u16 = self.program_counter.wrapping_add(signed_extension);
        let memory_address: u16 = self.memory.read(indirect_memory_address);
        let result: u16 = self.memory.read(memory_address);

        self.registers[dst as usize] = result;
        self.set_condition_codes(result);
    }

    fn load_offset(&mut self, operation: u16) {
        let dst: u8 = ((operation & 0x0E00) >> 9) as u8;
        let base: u8 = ((operation & 0x01C0) >> 6) as u8;
        let offset: u16 = self.sign_extension(operation & 0x003F, 6);

        let memory_address: u16 = self.registers[base as usize].wrapping_add(offset);

        let result: u16 = self.memory.read(memory_address);
        self.registers[dst as usize] = result;
        self.set_condition_codes(result);
    }

    fn load_immediate(&mut self, operation: u16) {
        let dst: u8 = ((operation & 0x0E00) >> 9) as u8;
        let signed_extension: u16 = self.sign_extension(operation & 0x01FF, 9);

        let memory_address: u16 = self.program_counter.wrapping_add(signed_extension);

        self.registers[dst as usize] = memory_address;
        self.set_condition_codes(memory_address);
    }

    fn store(&mut self, operation: u16) {
        let src: u8 = ((operation & 0x0E00) >> 9) as u8;
        let signed_extension: u16 = self.sign_extension(operation & 0x01FF, 9);

        let memory_address: u16 = self.program_counter.wrapping_add(signed_extension);

        self.memory
            .write(memory_address, self.registers[src as usize]);
    }

    fn store_indirect(&mut self, operation: u16) {
        let src: u8 = ((operation & 0x0E00) >> 9) as u8;
        let signed_extension: u16 = self.sign_extension(operation & 0x01FF, 9);

        let memory_address: u16 = self.program_counter.wrapping_add(signed_extension);
        let memory_address_content: u16 = self.memory.read(memory_address);

        self.memory
            .write(memory_address_content, self.registers[src as usize]);
    }

    fn store_offset(&mut self, operation: u16) {
        let src: u8 = ((operation & 0x0E00) >> 9) as u8;
        let base: u8 = ((operation & 0x01C0) >> 6) as u8;
        let offset: u16 = self.sign_extension(operation & 0x003F, 6);

        let memory_address: u16 = self.registers[base as usize].wrapping_add(offset);

        self.memory
            .write(memory_address, self.registers[src as usize]);
    }

    fn branch(&mut self, operation: u16) {
        let op_condition_codes: u8 = ((operation & 0x0E00) >> 9) as u8;
        let offset: u16 = self.sign_extension(operation & 0x01FF, 9);
        let condition_codes: u8 = (self.processor_status_register & 0x0007) as u8;

        match op_condition_codes & condition_codes {
            result if result > 0 => {
                self.program_counter = self.program_counter.wrapping_add(offset);
            }
            _ => {}
        }
    }

    fn jump(&mut self, operation: u16) {
        let base: u8 = ((operation & 0x01C0) >> 6) as u8;

        self.program_counter = self.registers[base as usize];
    }

    fn jump_register(&mut self, operation: u16) {
        let flag: u8 = ((operation & 0x0800) >> 11) as u8;
        self.registers[7] = self.program_counter;

        match flag {
            0 => {
                let base: u8 = ((operation & 0x01C0) >> 6) as u8;
                self.program_counter = self.registers[base as usize];
            }
            _ => {
                let offset: u16 = self.sign_extension(operation & 0x07FF, 11);
                self.program_counter = self.program_counter.wrapping_add(offset);
            }
        }
    }

    fn trap(&mut self, operation: u16) {
        let trap_vect: u8 = (operation & 0x00FF) as u8;

        match trap_vect {
            0x20 => {
                let mut buffer: [u8; 1] = [0 as u8; 1];
                std::io::stdin().read_exact(&mut buffer).unwrap();

                self.registers[0] = buffer[0].into();
            }
            0x21 => {
                print!("{}", (self.registers[0] as u8) as char);
            }
            0x22 => {
                let mut index = self.registers[0];
                while index < u16::MAX && self.memory[index] != 0 {
                    print!("{}", (self.memory[index] as u8) as char);
                    index = index + 1;
                }
            }
            0x23 => {
                print!("Please enter a character.");
                let char_value: u16 = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u16)
                    .unwrap();
                self.registers[0] = char_value;
            }
            0x24 => {
                let mut index = self.registers[0];

                while index < u16::MAX && self.memory[index] != 0 {
                    let word: u16 = self.memory[index];
                    let bytes = word.to_be_bytes();

                    print!("{}", bytes[1] as char);

                    if bytes[0] != 0 {
                        print!("{}", bytes[0] as char);
                    }

                    index = index + 1;
                }
            }
            0x25 => {
                print!("\nHALT\n");
                self.program_counter = u16::MAX;
            }
            _ => unreachable!("Invalid trap vector."),
        }
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
            x if (x >> 15) == 1 => self.processor_status_register |= 0b0000_0000_0000_0100,
            x if x == 0 => self.processor_status_register |= 0b0000_0000_0000_0010,
            _ => self.processor_status_register |= 0b0000_0000_0000_0001,
        };
    }
}

// Points to test file instead of directly testing here
#[cfg(test)]
#[path = "./cpu_test.rs"]
mod cpu_test;
