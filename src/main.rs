mod chip8;

use std::io;
fn init_chip8() -> chip8::Chip8 {
    let memory: [u8; 4096] = [0; 4096];
    let stack: [u16; 16] = [0; 16];
    let V: [u8; 16] = [0; 16];
    let keyboard: [u8; 16] = [0; 16];
    let display_board:[[u8; 64]; 32 ] = [[0; 64]; 32];

    chip8::Chip8 { memory, stack, V, time_reg: 0, sound_reg: 0, program_counter: 0x0200, stack_pointer: 0, I: 0 , keyboard, display_board}
}

fn main() {

    let mut chip8:chip8::Chip8 = init_chip8();

    loop {
        chip8.exec_cycle();
    }
}