mod cpu;
mod instruction;

use cpu::CPU;

fn main() {
    // println!("Hello, world!");

    let mut cpu = CPU::new();

    // Program:
    // MOV R0, 10
    // MOV R1, 5
    // SUB R2, R0, R1
    // PRINT R2
    // HALT

    cpu.memory[0] = 0x03; // MOV
    cpu.memory[1] = 0;    // R0
    cpu.memory[2] = 10;

    cpu.memory[3] = 0x03; // MOV
    cpu.memory[4] = 1;    // R1
    cpu.memory[5] = 5;

    cpu.memory[6] = 0x02; // SUB
    cpu.memory[7] = 2;    // R2
    cpu.memory[8] = 0;    // R0
    cpu.memory[9] = 1;    // R1

    cpu.memory[10] = 0x04; // PRINT
    cpu.memory[11] = 2;    // R2

    cpu.memory[12] = 0xFF; // HALT

    while !cpu.halted {
        cpu.step();
    }
}
