mod chip8;

use std::io;


fn main() {

    let mut chip8:chip8::Chip8 = chip8::init_chip8();

    loop {
        chip8.exec_cycle();
    }
}