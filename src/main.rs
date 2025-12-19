mod cpu;
mod instruction;

use cpu::CPU;

fn main() {
    // println!("Hello, world!");

    let mut cpu = CPU::new();

    // Program:
    // MOV R0, 42
    // STORE R0, 0x1000
    // LOAD R1, 0x1000
    // PRINT R1
    // HALT

    cpu.memory[0] = 0x03; // MOV
    cpu.memory[1] = 0;    // R0
    cpu.memory[2] = 42;

    cpu.memory[3] = 0x06; // STORE
    cpu.memory[4] = 0;    // R0
    cpu.memory[5] = 0x00;
    cpu.memory[6] = 0x10;

    cpu.memory[7] = 0x05;   // LOAD
    cpu.memory[8] = 1;      // R1
    cpu.memory[9] = 0x00;
    cpu.memory[10] = 0x10;

    cpu.memory[11] = 0x04; // PRINT
    cpu.memory[12] = 1;    // R1

    cpu.memory[13] = 0xFF; // HALT

    while !cpu.halted {
        cpu.step();
    }
}
