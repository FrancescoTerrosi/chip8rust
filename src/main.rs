mod chip8;

use std::io;

fn main() {
    let mut memory: [i8; 4096] = [0; 4096];
    let mut stack: [i16; 16] = [0; 16];
    let mut v: [i8; 16] = [0; 16];

    let mut chip8 = Chip8 { memory, stack, v, time_reg: 0, sound_reg: 0, program_counter: 0x0200, stack_pointer: 0 };

    loop {
        chip8.execution_cycle();
    }
}