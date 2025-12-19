mod cpu;
mod instruction;

use cpu::CPU;

fn main() {
    // println!("Hello, world!");

    let mut cpu = CPU::new();

    // Program:
    // MOV R0, 5
    // MOV R1, 1
    // LOOP:
    // PRINT R0
    // SUB R0, R0, R1
    // JNZ LOOP
    // HALT

    cpu.memory[0x0000] = 0x03; // MOV
    cpu.memory[0x0001] = 0;    // R0
    cpu.memory[0x0002] = 5;

    cpu.memory[0x0003] = 0x03; // MOV
    cpu.memory[0x0004] = 1;    // R1
    cpu.memory[0x0005] = 1;

    cpu.memory[0x0006] = 0x04; // PRINT
    cpu.memory[0x0007] = 0;    // R0

    cpu.memory[0x0008] = 0x02; // SUB
    cpu.memory[0x0009] = 0;    // R0
    cpu.memory[0x000A] = 0;    // R0
    cpu.memory[0x000B] = 1;    // R1

    cpu.memory[0x000C] = 0x09; // JNZ
    cpu.memory[0x000D] = 0x06;
    cpu.memory[0x000E] = 0x00;

    cpu.memory[0x000F] = 0xFF; // HALT

    while !cpu.halted {
        cpu.step();
    }
}
