use std::env;

use crate::cpu::CPU;

mod cpu;
mod memory;

fn main() {
    // Collect file path
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    println!("Attempting to execute program: {}", file_path);

    let mut cpu: CPU = CPU::new();
    cpu.execute_program(file_path);
}
