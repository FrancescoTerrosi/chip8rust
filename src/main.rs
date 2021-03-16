mod chip8;

use std::io;

fn main() {
    let mut memory: [u8; 4096] = [0; 4096];
    let mut stack: [u16; 16] = [0; 16];
    let mut V: [u8; 16] = [0; 16];
    let mut keyboard: [u8; 16] = [0; 16];
    let mut display_board:[[u8; 64]; 32 ] = [[0; 64]; 32];

    for i in 0..32 {

        println!("{:#?}", display_board[i]);

    }

    let mut chip8 = chip8::Chip8 { memory, stack, V, time_reg: 0, sound_reg: 0, program_counter: 0x0200, stack_pointer: 0, I: 0 , keyboard, display_board};

    loop {
        chip8.exec_cycle();
    }
}