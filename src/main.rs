mod cpu;
mod instruction;
mod assembler;

use cpu::CPU;
use assembler::assemble;

fn main() {
    // println!("Hello, world!");

    let source = r#"
        mov r0, 5
        push r0
        call double
        print r2
        halt

    double:
        loadr r1, FP, 8
        add r2, r1, r1
        ret
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
