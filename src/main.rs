mod cpu;
mod instruction;
mod assembler;

use cpu::CPU;
use assembler::assemble;

fn main() {
    // println!("Hello, world!");

    let source = r#"
        mov r0, 10
        call fib
        print r0
        halt

    fib:
        mov r1, 0
        sub r3, r0, r1
        jz ret_case

        mov r1, 1
        sub r3, r0, r1
        jz ret_case

        push r0
        
        mov r1, 1
        sub r0, r0, r1
        call fib
        
        movr r4, r0
        pop r0
        
        push r4
        
        mov r1, 2
        sub r0, r0, r1
        call fib
        
        pop r4
        add r0, r0, r4
        ret

    ret_case:
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
