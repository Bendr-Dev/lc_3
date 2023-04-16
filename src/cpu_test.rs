use crate::cpu::CPU;

//
// Initialization
//

#[test]
fn test_init_cpu() {
    let cpu: CPU = CPU::new();

    assert_eq!(cpu.program_counter, 0x3000);
    assert_eq!(cpu.processor_status_register, 0b0000_0010);
    assert_eq!(cpu.registers, [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]);
    assert_eq!(cpu.memory[0x3000], 0x0);
}

//
// Operations
//

// Data movement instructions

#[test]
fn test_store_operation() {
    let mut cpu: CPU = CPU::new();
    cpu.registers[0] = 0xF0BB;
    let operation: u16 = 0b0011_0000_0000_0010;

    cpu.memory[0x3000] = operation;
    cpu.tick();

    assert_eq!(cpu.memory[0x3003], 0xF0BB);
}

#[test]
fn test_store_indirect_operation() {
    let mut cpu: CPU = CPU::new();
    let operation: u16 = 0b1011_0100_0000_0001;

    cpu.registers[2] = 0x3400;

    cpu.memory[0x3000] = operation;
    cpu.memory[0x3002] = 0x3500;
    cpu.tick();

    assert_eq!(cpu.memory[0x3500], 0x3400);
}

#[test]
fn test_store_offset_operation() {
    let mut cpu: CPU = CPU::new();
    let operation: u16 = 0b0111_0100_0000_0001;

    cpu.registers[0] = 0x3100;
    cpu.registers[2] = 0x3400;

    cpu.memory[0x3000] = operation;
    cpu.tick();

    assert_eq!(cpu.memory[0x3101], 0x3400);
}

#[test]
fn test_load_operation() {
    let mut cpu: CPU = CPU::new();
    cpu.memory[0x3003] = 0xF0BB;
    let operation: u16 = 0b0010_0000_0000_0010;

    cpu.memory[0x3000] = operation;
    cpu.tick();

    assert_eq!(cpu.registers[0], 0xF0BB);
}

#[test]
fn test_load_imm_operation() {
    let mut cpu: CPU = CPU::new();
    let operation: u16 = 0b1110_1110_1111_1111;

    cpu.memory[0x3000] = operation;
    cpu.tick();

    assert_eq!(cpu.registers[7], 0x3100);
}

#[test]
fn test_load_indirect_operation() {
    let mut cpu: CPU = CPU::new();
    let operation: u16 = 0b1010_0010_1111_1111;

    cpu.memory[0x3100] = 0x3400;
    cpu.memory[0x3400] = 0x0005;

    cpu.memory[0x3000] = operation;
    cpu.tick();

    assert_eq!(cpu.registers[1], 0x0005);
}

#[test]
fn test_load_imm_operation_subtraction() {
    let mut cpu: CPU = CPU::new();
    let operation: u16 = 0b1110_0011_1111_1101;

    cpu.program_counter = 0x30F6;
    assert_eq!(cpu.program_counter, 0x30F6);

    cpu.memory[0x30F6] = operation;

    cpu.tick();

    assert_eq!(cpu.registers[1], 0x30F4);
}

#[test]
fn test_load_offset_operation() {
    let mut cpu: CPU = CPU::new();
    let operation: u16 = 0b0110_0100_0000_0001;

    cpu.registers[0] = 0x3100;
    cpu.memory[0x3101] = 0xFFF5;

    cpu.memory[0x3000] = operation;
    cpu.tick();

    assert_eq!(cpu.registers[2], 0xFFF5);
}

// Operate instructions

#[test]
fn test_not_operation() {
    let mut cpu: CPU = CPU::new();
    let operation: u16 = 0b1001_0010_0011_1111;

    cpu.memory[0x3000] = operation;
    cpu.tick();

    assert_eq!(cpu.registers[0], 0x0);
    assert_eq!(cpu.registers[1], 0xFFFF);
}

#[test]
fn test_and_operation() {
    let mut cpu: CPU = CPU::new();
    cpu.registers[0] = 0x0FF0;
    cpu.registers[2] = 0x0F0F;

    let operation: u16 = 0b0101_0010_0000_0010;

    cpu.memory[0x3000] = operation;
    cpu.tick();

    assert_eq!(cpu.registers[0], 0x0FF0);
    assert_eq!(cpu.registers[1], 0x0F00);
    assert_eq!(cpu.registers[2], 0x0F0F);
}

#[test]
fn test_and_imm_operation() {
    let mut cpu: CPU = CPU::new();
    cpu.registers[0] = 0x0FFF;

    let operation: u16 = 0b0101_0010_0010_1011;

    cpu.memory[0x3000] = operation;
    cpu.tick();

    assert_eq!(cpu.registers[0], 0x0FFF);
    assert_eq!(cpu.registers[1], 0x000B);
}

#[test]
fn test_add_operation() {
    let mut cpu: CPU = CPU::new();
    cpu.registers[0] = 0x000A;
    cpu.registers[2] = 0xFFFB;

    let operation: u16 = 0b0001_0010_0000_0010;

    cpu.memory[0x3000] = operation;
    cpu.tick();

    assert_eq!(cpu.registers[0], 0x000A);
    assert_eq!(cpu.registers[1], 0x0005);
    assert_eq!(cpu.registers[2], 0xFFFB);
}

#[test]
fn test_add_imm_operation() {
    let mut cpu: CPU = CPU::new();
    cpu.registers[0] = 0x30F4;

    let operation: u16 = 0b0001_0010_0011_1111;

    cpu.memory[0x3000] = operation;
    cpu.tick();

    assert_eq!(cpu.registers[1], 0x30F3);
}

// Control instructions

#[test]
fn test_branch_operation() {
    let mut cpu: CPU = CPU::new();
    let operation: u16 = 0b0000_1110_0000_1111;

    cpu.memory[0x3000] = operation;
    cpu.tick();

    assert_eq!(cpu.program_counter, 0x3010);
}

#[test]
fn test_jump_operation() {
    let mut cpu: CPU = CPU::new();
    let operation: u16 = 0b1100_0000_0100_0000;

    cpu.registers[1] = 0x3400;
    cpu.memory[0x3000] = operation;
    cpu.tick();

    assert_eq!(cpu.program_counter, 0x3400);
}

#[test]
fn test_trap_print_operation() {
    let mut cpu: CPU = CPU::new();
    let operation: u16 = 0b1111_0000_0010_0001;

    cpu.registers[0] = 0x0041;
    cpu.memory[0x3000] = operation;
    cpu.tick();

    assert_eq!(((cpu.registers[0] & 0x00FF) as u8) as char, 'A');
}

#[test]
fn test_trap_print_byte_char_operation() {
    let mut cpu: CPU = CPU::new();
    let operation: u16 = 0b1111_0000_0010_0010;

    cpu.registers[0] = 0x3100;
    cpu.memory[0x3100] = 0x54;
    cpu.memory[0x3101] = 0x65;
    cpu.memory[0x3102] = 0x73;
    cpu.memory[0x3103] = 0x74;
    cpu.memory[0x3000] = operation;
    cpu.tick();
}

#[test]
fn test_trap_print_nibble_char_operation() {
    let mut cpu: CPU = CPU::new();
    let operation: u16 = 0b1111_0000_0010_0100;

    cpu.registers[0] = 0x3100;
    cpu.memory[0x3100] = 0x6548;
    cpu.memory[0x3101] = 0x6C6C;
    cpu.memory[0x3102] = 0x216F;
    cpu.memory[0x3000] = operation;
    cpu.tick();
}

#[test]
#[should_panic]
fn test_trap_halt_operation() {
    let mut cpu: CPU = CPU::new();
    let operation: u16 = 0b1111_0000_0010_0101;

    cpu.memory[0x3000] = operation;
    cpu.tick();
}

#[test]
#[should_panic]
fn test_trap_invalid_operation() {
    let mut cpu: CPU = CPU::new();
    let operation: u16 = 0b1111_0000_1111_1111;

    cpu.memory[0x3000] = operation;
    cpu.tick();
}

//
// Helper functions
//

#[test]
fn test_signed_extension() {
    let cpu: CPU = CPU::new();
    let neg_bits: u16 = cpu.sign_extension(0b1_1101, 5) as u16;
    let pos_bits: u16 = cpu.sign_extension(0b01001_1101, 9) as u16;

    assert_eq!(neg_bits, 0b1111_1111_1111_1101);
    assert_eq!(pos_bits, 0b0000_0000_1001_1101);
}

#[test]
fn test_update_condition_codes() {
    let mut cpu: CPU = CPU::new();
    cpu.processor_status_register = 0b1111_1111_1111_0010;
    cpu.set_condition_codes(0x0FF0);

    assert_eq!(cpu.processor_status_register, 0xFFF1);

    cpu.set_condition_codes(0xF011);

    assert_eq!(cpu.processor_status_register, 0xFFF4);

    cpu.set_condition_codes(0x0);

    assert_eq!(cpu.processor_status_register, 0xFFF2);
}

//
// Examples (Combination of instructions)
//
