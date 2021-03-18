
use rand::Rng;

pub struct Chip8 {
    pub memory: [u8; 4096],  // 0x000 to 0x1FF are not to be used --> most programs start at 0x200
    pub stack: [u16; 16],
    pub V: [u8; 16],    // 16 8-bit registers V0 - VF
    pub I: u16,
    pub time_reg: u8,
    pub sound_reg: u8,
    pub program_counter: usize,
    pub stack_pointer: usize,
    pub keyboard: [u8; 16],
    pub display_board:[[u8; 64]; 32 ]
}

impl Chip8 {

    /* TODO:
    *   Check if this works
    */

    fn draw_sprite(&mut self, n:usize, x: usize, y:usize) {
        let mut sprites = vec![0; n].as_mut_slice();
        let mut inverted_pixel:u8 = 0;
        for i in 0..n {
            if self.display_board[x][y] == 1 && self.memory[self.I + i*8] == 1 {
                inverted_pixel = 1;
            }
            self.display_board[(x+i) % 64][(y+i) % 32] = self.display_board[(x+i) % 64][(y+i) % 32] ^ self.memory[self.I + i*8];
        }
        self.V[0xF] = inverted_pixel;
    }

    fn fetch(&mut self) -> u16 {

        let mut instruction:u16 = self.memory[self.program_counter] as u16;
        instruction = (instruction << 8) | (self.memory[self.program_counter+1] as u16);
        self.program_counter += 2;
        return instruction;
    }

    fn execute(&mut self, instruction: u16) {

        //  decode instruction from memory

        let opcode:u16 = (instruction & 0x1000) as u16;

        let nnn:usize = (instruction & 0x0FFF) as usize;
        let kk:u8 = (instruction & 0x00FF) as u8;
        let x:usize = ((instruction >> 8) & 0x0F00) as usize;
        let y:usize = ((instruction >> 4) & 0x00F0) as usize;
        let n:u8 = (instruction & 0x000F) as u8;

        //  execute

        match opcode {
            0x0000 => {

                match kk {
                    0x00E0 => {
                        self.display_board = [[0; 64]; 32];
                        self.program_counter += 2;
                    },

                    0x00EE => {
                        self.program_counter = self.stack[self.stack_pointer] as usize;
                        self.stack_pointer -= 2;
                    },
                    _ => {
                        panic!("NOOOOOOO");
                    }
                }

            },

            0x1000 => {
                self.program_counter = nnn;
            },

            0x2000 => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer] = self.program_counter as u16 ;
                self.program_counter = nnn;
            },

            0x3000 => {
                if self.V[x] == kk {
                    self.program_counter += 4;
                }
            },

            0x4000 => {
                if self.V[x] != kk {
                    self.program_counter += 4;
                }
            },

            0x5000 => {
                if self.V[x] == self.V[y] {
                    self.program_counter += 4;
                }
            },

            0x6000 => {
                self.V[x] = kk;
                self.program_counter += 2;
            },

            0x7000 => {
                self.V[x] += kk;
                self.program_counter += 2;
            },

            0x8000 => {
                match n {
                    0x0000 => {
                        self.V[x] = self.V[y];
                        self.program_counter += 2;
                    },

                    0x0001 => {
                        self.V[x] = self.V[x] | self.V[y];
                        self.program_counter += 2;
                    },

                    0x0002 => {
                        self.V[x] = self.V[x] & self.V[y];
                        self.program_counter += 2;
                    },

                    0x0003 => {
                        self.V[x] = self.V[x] ^ self.V[y];
                        self.program_counter += 2;
                    },

                    0x0004 => {
                        let temp:i16 = (self.V[x] + self.V[y]).into();

                        if temp > 255 {
                            self.V[0xF] = 1;
                            self.V[x] = (temp & 0x00FF) as u8;
                        }
                        self.program_counter += 2;
                    },

                    0x0005 => {
                        if self.V[x] > self.V[y] {
                            self.V[0xF] = 1;
                        }
                        self.V[x] -= self.V[y];
                        self.program_counter += 2;
                    },

                    0x0006 => {
                        if self.V[x] & 0x0001 == 1 {
                            self.V[0xF] = 1;
                        }
                        self.V[x] = self.V[x] >> 1;
                        self.program_counter += 2;
                    },

                    0x0007 => {
                        if self.V[y] > self.V[x] {
                            self.V[0xF] = 1;
                        }
                        self.V[x] = self.V[y] - self.V[x];
                        self.program_counter += 2;
                    },

                    0x000E => {
                        if self.V[x] & 0x10 == 0x10 {
                            self.V[0xF] = 1;
                        }
                        self.V[x] = self.V[x] << 1;
                        self.program_counter += 2;
                    },

                    _ => {
                        panic!("ARGGGG");
                    }
                };
            },

            0x9000 => {
                if self.V[x] != self.V[y] {
                    self.program_counter += 4;
                }
            },

            0xA000 => {
                self.I = nnn as u16;
                self.program_counter += 2;
            },

            0xB000 => {
                self.program_counter = (self.V[0] as usize + nnn).into();
            },

            0xC000 => {
                let rand_numb = rand::thread_rng().gen_range(0..=255);
                self.V[x] = rand_numb & kk;
                self.program_counter += 2;
            },

            0xD000 => {

                self.draw_sprite(n as usize, self.V[x] as usize, self.V[y] as usize);
                self.program_counter += 2;
            },

            0xE000 => {

                match kk {

                    0x009E => {

                        // KEYBOARD STUFF

                        self.program_counter += 2;
                    },

                    0x00A1 => {
                        // KEYBOARD STUFF
                        self.program_counter += 2;
                    },

                    _ => {
                        panic!("AIAIAIIIIIIIIIIIIII");
                    }
                }

            },

            0xF000 => {
                match kk {

                    0x0007 => {
                        self.V[x] = self.time_reg;
                        self.program_counter += 2;
                    },

                    0x000A => {

                        // wait for key press and do stuff

                        self.program_counter += 2;
                    },

                    0x0015 => {

                        self.time_reg = self.V[x];

                        self.program_counter += 2;
                    },

                    0x0018 => {
                        self.sound_reg = self.V[x];
                        self.program_counter += 2;
                    },

                    0x001E => {
                        self.I += self.V[x] as u16;
                        self.program_counter += 2;
                    },

                    0x0029 => {

                        // graphics stuff

                        self.program_counter += 2;
                    },

                    0x0033 => {
                        // TODO
                    },

                    0x0055 => {
                        // TODO
                    },

                    0x0065 => {
                        // TODO
                    },

                    _ => {
                        panic!("PUUUUNNOOOOOOOUZZZ");
                    }


                }
            },

            _ => {
                panic!("AAAAAHH");
            }
        }
    }

    pub fn exec_cycle(&mut self) {
        let instruction = self.fetch();
        self.execute(instruction);
    }

}