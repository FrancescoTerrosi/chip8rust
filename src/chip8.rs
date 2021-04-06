
use rand::Rng;

fn init_memory() -> [u8; 4096] {
    let mut memory: [u8; 4096] = [0; 4096];

    memory[0]=0xF0;
    memory[1]=0x90;
    memory[2]=0x90;
    memory[3]=0x90;
    memory[4]=0xF0;

    memory[5]=0x20;
    memory[6]=0x60;
    memory[7]=0x20;
    memory[8]=0x20;
    memory[9]=0x70;

    memory[10]=0xF0;
    memory[11]=0x10;
    memory[12]=0xF0;
    memory[13]=0x80;
    memory[14]=0xF0;

    memory[15]=0xF0;
    memory[16]=0x10;
    memory[17]=0xF0;
    memory[18]=0x10;
    memory[19]=0xF0;

    memory[20]=0x90;
    memory[21]=0x90;
    memory[22]=0xF0;
    memory[23]=0x10;
    memory[24]=0x10;

    memory[25]=0xF0;
    memory[26]=0x80;
    memory[27]=0xF0;
    memory[28]=0x10;
    memory[29]=0xF0;

    memory[30]=0xF0;
    memory[31]=0x80;
    memory[32]=0xF0;
    memory[33]=0x90;
    memory[34]=0xF0;

    memory[35]=0xF0;
    memory[36]=0x10;
    memory[37]=0x20;
    memory[38]=0x40;
    memory[39]=0x40;

    memory[40]=0xF0;
    memory[41]=0x90;
    memory[42]=0xF0;
    memory[43]=0x90;
    memory[44]=0xF0;

    memory[45]=0xF0;
    memory[46]=0x90;
    memory[47]=0xF0;
    memory[48]=0x10;
    memory[49]=0xF0;

    memory[50]=0xF0;
    memory[51]=0x90;
    memory[52]=0xF0;
    memory[53]=0x90;
    memory[54]=0x90;

    memory[55]=0xE0;
    memory[56]=0x90;
    memory[57]=0xE0;
    memory[58]=0x90;
    memory[59]=0xE0;

    memory[60]=0xF0;
    memory[61]=0x80;
    memory[62]=0x80;
    memory[63]=0x80;
    memory[64]=0xF0;

    memory[65]=0xE0;
    memory[66]=0x90;
    memory[67]=0x90;
    memory[68]=0x90;
    memory[69]=0xE0;

    memory[70]=0xF0;
    memory[71]=0x80;
    memory[72]=0xF0;
    memory[73]=0x80;
    memory[74]=0xF0;

    memory[75]=0xF0;
    memory[76]=0x80;
    memory[77]=0xF0;
    memory[78]=0x80;
    memory[79]=0x80;

    return memory;

}

pub fn init_chip8() -> Chip8 {
    let memory = init_memory();
    let stack: [u16; 16] = [0; 16];
    let V: [u8; 16] = [0; 16];
    let keyboard: [u8; 16] = [0; 16];
    let display_board:[[u8; 64]; 32 ] = [[0; 64]; 32];

    Chip8 { memory, stack, V, time_reg: 0, sound_reg: 0, program_counter: 0x0200, stack_pointer: 0, I: 0 , keyboard, display_board}
}

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
        let mut inverted_pixel:u8 = 0;
        for i in 0..n {
            if self.display_board[(x+i) % 64][(y+i) % 32] == 1 && self.memory[self.I as usize + i*8] == 1 {
                inverted_pixel = 1;
            }
            self.display_board[(x+i) % 64][(y+i) % 32] = self.display_board[(x+i) % 64][(y+i) % 32] ^ self.memory[self.I as usize + i*8];
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
                    self.program_counter += 2;
                }
                self.program_counter += 2;
            },

            0x4000 => {
                if self.V[x] != kk {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
            },

            0x5000 => {
                if self.V[x] == self.V[y] {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
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
                    self.program_counter += 2;
                }
                self.program_counter += 2;
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

                        if self.keyboard[self.V[x] as usize] == 1 {
                            self.program_counter += 2;
                        }
                        self.program_counter += 2;
                    },

                    0x00A1 => {
                        if self.keyboard[self.V[x] as usize] != 1 {
                            self.program_counter += 2;
                        }
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

                        for i in 0..self.keyboard.len() {
                            if self.keyboard[i] == 1 {
                                self.V[x] = self.keyboard[i];
                                self.program_counter += 2;
                            }
                        }

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

                        self.I = self.V[x].into();

                        self.program_counter += 2;
                    },

                    0x0033 => {
                        self.memory[self.I as usize] = (x/100) as u8;
                        self.memory[(self.I as usize) + 1] = ((x/10) % 10) as u8;
                        self.memory[(self.I as usize) + 1] = (x % 10) as u8;
                        self.program_counter += 2;
                    },

                    0x0055 => {
                        for i in 0..x {
                            self.memory[self.I as usize + i] = self.V[i];
                        }
                        self.program_counter += 2;
                    },

                    0x0065 => {
                        for i in 0..x {
                            self.V[i] = self.memory[self.I as usize + i];
                        }
                        self.program_counter += 2;
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