use std::io;
use std::ops::BitXor;
use std::default::default;
use rand::Rng;

pub struct Chip8 {
    pub memory: [i8; 4096],  // 0x000 to 0x1FF are not to be used --> most programs start at 0x200
    pub stack: [i16; 16],
    pub V: [i8; 16],    // 16 8-bit registers V0 - VF
    pub I: i16;
    pub time_reg: i8,
    pub sound_reg: i8,
    pub program_counter: i16,
    pub stack_pointer: i8
}

impl Chip8 {

    fn fetch(&mut self) -> i16 {
        let instruction = self.memory[self.program_counter];
        self.program_counter += 1;
        return instruction;
    }

    fn execute(&mut self, instruction: i16) {

        //  decode instruction from memory

        let opcode = instruction & 0x1000;

        let nnn = instruction & 0x0FFF;
        let kk = instruction & 0x00FF;
        let x = (instruction >> 8) & 0x0F00;
        let y = (instruction >> 4) & 0x00F0;
        let n = instruction & 0x000F;

        //  execute

        match opcode {
            0x0000 => {

                match kk {
                    0x00E0 => {
                        //clear graphics
                        self.program_counter += 1;
                    },

                    0x00EE => {
                        self.program_counter = self.stack[self.stack_pointer];
                        self.stack_pointer -= 1;
                    },
                    default => {
                        panic!("NOOOOOOO");
                    }
                }

            },

            0x1000 => {
                self.program_counter = nnn;
            },

            0x2000 => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer] = self.program_counter;
                self.program_counter = nnn;
            },

            0x3000 => {
                if self.V[x] == kk {
                    self.program_counter += 2;
                }
            },

            0x4000 => {
                if self.V[x] != kk {
                    self.program_counter += 2;
                }
            },

            0x5000 => {
                if self.V[x] == self.V[y] {
                    self.program_counter += 2;
                }
            },

            0x6000 => {
                self.V[x] = kk;
                self.program_counter += 1;
            },

            0x7000 => {
                self.V[x] += kk;
                self.program_counter += 1;
            },

            0x8000 => {
                match n {
                    0x0000 => {
                        self.V[x] = self.V[y];
                        self.program_counter += 1;
                    },

                    0x0001 => {
                        self.V[x] = self.V[x] | self.V[y];
                        self.program_counter += 1;
                    },

                    0x0002 => {
                        self.V[x] = self.V[x] & self.V[y];
                        self.program_counter += 1;
                    },

                    0x0003 => {
                        self.V[x] = self.V[x] ^ self.V[y];
                        self.program_counter += 1;
                    },

                    0x0004 => {
                        let temp:i16 = self.V[x] + self.V[y];

                        if temp > 255 {
                            self.V[0xF] = 1;
                            self.V[x] = (temp & 0x00FF) as i8;
                        }
                        self.program_counter += 1;
                    },

                    0x0005 => {
                        if self.V[x] > self.V[y] {
                            self.V[0xF] = 1;
                        }
                        self.V[x] -= self.V[y];
                        self.program_counter += 1;
                    },

                    0x0006 => {
                        if self.V[x] & 0x0001 == 1 {
                            self.V[0xF] = 1;
                        }
                        self.V[x] = self.V[x] >> 1;
                        self.program_counter += 1;
                    },

                    0x0007 => {
                        if self.V[y] > self.V[x] {
                            self.V[0xF] = 1;
                        }
                        self.V[x] = self.V[y] - self.V[x];
                        self.program_counter += 1;
                    },

                    0x000E => {
                        if self.V[x] & 0x1000 == 0x1000 {
                            self.V[0xF] = 1;
                        }
                        self.V[x] = self.V[x] << 1;
                        self.program_counter += 1;
                    },

                    default => {
                        panic!("ARGGGG");
                    }
                };
            },

            0x9000 => {
                if self.V[x] != self.V[y] {
                    self.program_counter += 2;
                }
            },

            0xA000 => {
                self.I = nnn;
                self.program_counter += 1;
            },

            0xB000 => {
                self.program_counter = self.V[0] + nnn;
            },

            0xC000 => {
                let rand_numb = rand::thread_rng().gen_range(0, 256);
                self.V[x] = rand_numb & kk
                self.program_counter += 1;
            },

            0xD000 => {

                // DISPLAY STUFF

                self.program_counter += 1;
            },

            0xE000 => {

                match kk {

                    0x009E => {

                        // KEYBOARD STUFF

                        self.program_counter += 1;
                    },

                    0x00A1 => {
                        // KEYBOARD STUFF
                        self.program_counter += 1;
                    },

                    default => {
                        panic!("AIAIAIIIIIIIIIIIIII");
                    }
                }

            },

            0xF000 => {
                match kk {

                    0x0007 => {
                        self.V[x] = self.time_reg;
                        self.program_counter += 1;
                    },

                    0x000A => {

                        // wait for key press and do stuff

                        self.program_counter += 1;
                    },

                    0x0015 => {

                        self.time_reg = self.V[x];

                        self.program_counter += 1;
                    },

                    0x0018 => {
                        self.sound_reg = self.V[x];
                        self.program_counter += 1;
                    },

                    0x001E => {
                        self.I += self.V[x];
                        self.program_counter += 1;
                    },

                    0x0029 => {

                        // graphics stuff

                        self.program_counter += 1;
                    },

                    0x0033 => {
                        // TODO
                    },

                    0x0055 => {
                        // TODO
                    },

                    0x0065 => {
                        // TODO
                    }


                }
            },

            default => {
                panic!("AAAAAHH");
            }
        }
    }

    fn execution_cycle() {

    }

}