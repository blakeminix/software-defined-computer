mod cpu;
mod instruction;
mod assembler;

use cpu::CPU;
use assembler::assemble;

fn main() {
    // println!("Hello, world!");

    let source = r#"
        MOV R0, 10
        MOV R1, 5
        MOV R2, 0
        MOV R3, 0
        ADD R2, R0, R1
        PRINT R2
        SUB R2, R1, R0
        PRINT R2
        STORE R2, 0x2000
        LOAD R4, 0x2000
        PRINT R4
        MOV R5, 20
        JZ skip
        MOV R6, 1
    skip:
        PRINT R5
        MOV R5, 5
    loop:
        SUB R5, R5, R1
        JNZ loop
        PRINT R5
        JMP end
        MOV R0, 999
        PRINT R0
    end:
        HALT
    "#;

    let mut cpu = CPU::new();
    let program = assemble(source);

    for (i, byte) in program.iter().enumerate() {
        cpu.memory[i] = *byte;
    }

    while !cpu.halted {
        cpu.step();
    }
}
