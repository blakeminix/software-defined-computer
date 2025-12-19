mod cpu;
mod instruction;
mod assembler;

use cpu::CPU;
use assembler::assemble;

fn main() {
    // println!("Hello, world!");

    let source = r#"
        mov r0, 10
        call add
        print r0
        halt
    add:
        add r0, r0, r0
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
