extern crate termios;

use std::env;
use termios::*;

use crate::cpu::CPU;

mod cpu;
mod memory;

fn main() {
    // Collect file path
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    // Unix-based os terminal configuration to make it interactive for the VM
    let stdin = 0;
    let termios = termios::Termios::from_fd(stdin).unwrap();

    // Make mutable and copy
    let mut new_termios = termios.clone();
    new_termios.c_iflag &= IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON;
    new_termios.c_lflag &= !(ICANON | ECHO); // No echo and canonical mode

    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();

    println!("Attempting to execute program: {}", file_path);

    let mut cpu: CPU = CPU::new();
    cpu.execute_program(file_path);

    // reset the stdin to
    // original termios data
    tcsetattr(stdin, TCSANOW, &termios).unwrap();
}
