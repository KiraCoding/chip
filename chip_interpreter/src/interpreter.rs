use rand::random;

pub enum InterpreterEvent {
    Audio,
    Opcode(u16),
}

pub struct Interpreter {
    pc: u16,
    index: u16,
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    v: [u8; 16],
    stack: [u16; 16],
    memory: [u8; 4096],
    pub screen_buffer: [u8; 64 * 32],
}

impl Interpreter {
    pub fn load(&mut self, program: &[u8]) {
        *self = Default::default();
        self.memory[self.pc as usize..self.pc as usize + program.len()].copy_from_slice(program);
    }

    pub fn cycle(&mut self) -> InterpreterEvent {
        let opcode = u16::from_be_bytes([
            self.memory[self.pc as usize],
            self.memory[(self.pc + 1) as usize],
        ]);

        #[inline(always)]
        const fn nnn(opcode: u16) -> u16 {
            opcode & 0x0FFF
        }

        #[inline(always)]
        const fn nn(opcode: u16) -> u8 {
            (opcode & 0x00FF) as u8
        }

        #[inline(always)]
        const fn n(opcode: u16) -> usize {
            (opcode & 0x000F) as usize
        }

        #[inline(always)]
        const fn x(opcode: u16) -> usize {
            ((opcode & 0x0F00) >> 8) as usize
        }

        #[inline(always)]
        const fn y(opcode: u16) -> usize {
            ((opcode & 0x00F0) >> 4) as usize
        }

        match opcode {
            0x00E0 => {
                self.screen_buffer = [0; 64 * 32];
                self.pc += 2;
            }

            0x00EE => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
                self.pc += 2;
            }

            // JMP NNN | 1NNN
            0x1000..=0x1FFF => self.pc = nnn(opcode),

            // CALL NNN | 2NNN
            0x2000..=0x2FFF => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn(opcode);
            }

            // JEQ VX, NN | 3XNN
            0x3000..=0x3FFF => {
                let x = x(opcode);
                let data = nn(opcode);

                self.pc += if self.v[x] == data { 4 } else { 2 };
            }

            // JNE VX, NN | 4XNN
            0x4000..=0x4FFF => {
                let x = x(opcode);
                let data = nn(opcode);

                self.pc += if self.v[x] != data { 4 } else { 2 };
            }

            // JEQ VX, VY | 5XY0
            0x5000..=0x5FF0 => {
                let x = x(opcode);
                let y = y(opcode);

                self.pc += if self.v[x] == self.v[y] { 4 } else { 2 };
            }

            // MOV VX, NN | 6XNN
            0x6000..=0x6FFF => {
                let x = x(opcode);
                let data = nn(opcode);

                self.v[x] = data;
                self.pc += 2;
            }

            // MOV VX, VY | 8XY0
            0x8000..=0x8FF0 => {
                self.v[x(opcode)] = self.v[y(opcode)];
                self.pc += 2;
            }

            // OR VX, VY | 8XY1
            0x8001..=0x8FF1 => {
                self.v[x(opcode)] |= self.v[y(opcode)];
                self.pc += 2;
            }

            // AND VX, VY | 8XY2
            0x8002..=0x8FF2 => {
                self.v[x(opcode)] &= self.v[y(opcode)];
                self.pc += 2;
            }

            // XOR VX, VY | 8XY3
            0x8003..=0x8FF3 => {
                self.v[x(opcode)] ^= self.v[y(opcode)];
                self.pc += 2;
            }

            // ADC VX, VY | 8XY4
            0x8004..=0x8FF4 => {
                let x = x(opcode);
                let y = y(opcode);

                let (result, overflow) = self.v[x].overflowing_add(self.v[y]);

                self.v[0xF] = overflow as u8;
                self.v[x] = result;
                self.pc += 2;
            }

            // SBC VX, VY | 8XY5
            0x8005..=0x8FF5 => {
                let x = x(opcode);
                let y = y(opcode);

                let (result, overflow) = self.v[x].overflowing_sub(self.v[y]);

                self.v[0xF] = overflow as u8;
                self.v[x] = result;
                self.pc += 2;
            }

            // SHR VX | 8XY6
            0x8006..=0x8FF6 => {
                let x = x(opcode);

                self.v[0xF] = self.v[x] & 0x1;
                self.v[x] >>= 1;
                self.pc += 2;
            }

            // SUB VX, VY | 8XY7
            0x8007..=0x8FF7 => {
                let x = x(opcode);
                let y = y(opcode);

                let (result, overflow) = self.v[y].overflowing_sub(self.v[x]);

                self.v[0xF] = overflow as u8;
                self.v[x] = result;
                self.pc += 2;
            }

            // SHL VX | 8XYE
            0x800E..=0x8FFE => {
                let x = x(opcode);

                self.v[0xF] = (self.v[x] & 0x80) >> 7;
                self.v[x] <<= 1;
                self.pc += 2;
            }

            // JNE VX, VY | 9XY0
            0x9000..=0x9FF0 => {
                let x = x(opcode);
                let y = y(opcode);

                self.pc += if self.v[x] != self.v[y] { 4 } else { 2 };
            }

            // LOAD I | ANNN
            0xA000..=0xAFFF => {
                self.index = nnn(opcode);
                self.pc += 2;
            }

            // JMP V0, NNN | BNNN
            0xB000..=0xBFFF => self.pc = nnn(opcode) + self.v[0] as u16,

            // RAND VX, NN | CXNN
            0xC000..=0xCFFF => {
                let data = nn(opcode);

                self.v[x(opcode)] = data & random::<u8>();
                self.pc += 2;
            }

            // DRAW VX, VY, N | DXYN
            0xD000..=0xDFFF => {
                let vx = self.v[x(opcode)] as usize;
                let vy = self.v[y(opcode)] as usize;
                let height = n(opcode);

                self.v[0xF] = 0;

                for row in 0..height {
                    let pixel = self.memory[self.index as usize + row];

                    for col in 0..8 {
                        if (pixel & (1 << (7 - col))) != 0 {
                            let index = ((vy + row) % 32) * 64 + (vx + col) % 64;
                            let read = &mut self.screen_buffer[index];

                            self.v[0xF] |= *read & 1;
                            *read ^= 1;
                        }
                    }
                }

                self.pc += 2;
            }

            // KEY VX | EX9E
            0xE09E..=0xEF9E => {
                self.pc += 2; // TODO
            }

            // KEYNOT VX | EXA1
            0xE0A1..=0xEFA1 => {
                self.pc += 2; // TODO
            }

            // MOVDELAY VX | FX07
            _ if opcode & 0xF0FF == 0xF007 => {
                self.v[x(opcode)] = self.delay_timer;
                self.pc += 2;
            }

            // WAITKEY | FX0A
            _ if opcode & 0xF0FF == 0xF00A => {
                self.pc += 2; // TODO
            }

            // SET_DELAY VX | FX15
            _ if opcode & 0xF0FF == 0xF015 => {
                self.delay_timer = self.v[x(opcode)];
                self.pc += 2;
            }

            // SETSOUND VX | FX18
            _ if opcode & 0xF0FF == 0xF018 => {
                self.sound_timer = self.v[x(opcode)];
                self.pc += 2;
            }

            // ADD_TO_INDEX Vx | FX1E
            _ if opcode & 0xF0FF == 0xF01E => {
                self.index += self.v[x(opcode)] as u16;
                self.pc += 2;
            }

            // SET_SPRITE_ADDR Vx | FX29
            _ if opcode & 0xF0FF == 0xF029 => {
                let character = self.v[x(opcode)];

                self.index = character as u16 * 5;
                self.pc += 2;
            }

            // STORE_BCD VX | FX33
            _ if opcode & 0xF0FF == 0xF033 => {
                let value = self.v[x(opcode)];

                self.memory[self.index as usize] = value / 100;
                self.memory[self.index as usize + 1] = (value / 10) % 10;
                self.memory[self.index as usize + 2] = value % 10;

                self.pc += 2;
            }

            // REG_DUMP [I] VX | FX55
            _ if opcode & 0xF0FF == 0xF055 => {
                for index in 0..=x(opcode) {
                    self.memory[self.index as usize + index] = self.v[index];
                }

                self.pc += 2;
            }

            // REG_LOAD [I] VX | FX65
            _ if opcode & 0xF0FF == 0xF065 => {
                for index in 0..=x(opcode) {
                    self.v[index] = self.memory[self.index as usize + index];
                }

                self.pc += 2;
            }

            // REG_LOAD VX, I | FX65 ||NEW
            // _ if opcode & 0xF0FF == 0xF065 => {
            //     let start = self.index as usize;
            //     let end = start + x as usize;

            //     self.v[0..=x(opcode)].copy_from_slice(&self.memory[start..=end]);

            //     self.pc += 2;
            // }
            _ => panic!("Invalid opcode"),
        }

        if self.sound_timer > 0 {
            // Play the beep sound when sound_timer > 0
            self.sound_timer -= 1;
            return InterpreterEvent::Audio;
        } else {
            InterpreterEvent::Opcode(opcode)
        }
    }
}

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

impl Default for Interpreter {
    fn default() -> Self {
        let mut memory = [0; 4096];
        memory[0..FONT.len()].copy_from_slice(&FONT);

        Self {
            pc: 0x200,
            index: 0x200,
            memory,
            sp: Default::default(),
            delay_timer: Default::default(),
            sound_timer: Default::default(),
            v: Default::default(),
            stack: Default::default(),
            screen_buffer: [0; 64 * 32],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Interpreter;

    #[test]
    fn test_1nnn() {
        let mut interpreter = Interpreter::default();

        interpreter.load(&[0x12, 0x34]);
        interpreter.cycle();

        assert_eq!(interpreter.pc, 0x234);
    }

    #[test]
    fn test_2nnn() {
        let mut interpeter = Interpreter::default();

        interpeter.load(&[0x23, 0x45]);
        interpeter.cycle();

        assert_eq!(interpeter.sp, 1);
        assert_eq!(interpeter.pc, 0x345);
        assert_eq!(interpeter.stack[0], 0x200);
    }

    #[test]
    fn test_3xnn() {
        let mut interpreter = Interpreter::default();

        interpreter.load(&[0x32, 0xAB]);
        interpreter.v[2] = 0xAB;
        interpreter.cycle();
        assert_eq!(interpreter.pc, 0x200 + 4);

        interpreter.load(&[0x32, 0xAB]);
        interpreter.v[2] = 0xAB + 1;
        interpreter.cycle();
        assert_eq!(interpreter.pc, 0x200 + 2);
    }

    #[test]
    fn test_4xnn() {
        let mut interpreter = Interpreter::default();

        // Test case 1: When Vx == NN, the program counter should not skip (increase by 2)
        interpreter.load(&[0x32, 0x12]); // Set V1 to 0x12
        interpreter.v[2] = 0x12;
        interpreter.cycle();
        assert_eq!(interpreter.pc, 0x200 + 2);

        // Test case 2: When Vx != NN, the program counter should skip (increase by 4)
        interpreter.load(&[0x32, 0x34]); // Set V1 to 0x34
        interpreter.v[2] = 0x12;
        interpreter.cycle();
        assert_eq!(interpreter.pc, 0x200 + 4);
    }

    #[test]
    fn test_5xy0() {
        // Test the 5XY0 opcode when registers Vx and Vy are equal. (increase by 4)
        let mut interpreter = Interpreter::default();
        interpreter.load(&[0x52, 0x30]);
        interpreter.v[2] = 0x30;
        interpreter.v[3] = 0x30;
        interpreter.cycle();
        assert_eq!(interpreter.pc, 0x200 + 4);

        // Test the 5XY0 opcode when registers Vx and Vy are not equal. (increase by 2)
        let mut interpreter = Interpreter::default();
        interpreter.load(&[0x54, 0x50]);
        interpreter.v[4] = 0x50;
        interpreter.v[5] = 0x20;
        interpreter.cycle();
        assert_eq!(interpreter.pc, 0x200 + 2);
    }

    #[test]
    fn test_6xnn() {
        // Test the 6XNN opcode (MOV VX, NN) to load a value into a register.
        let mut interpreter = Interpreter::default();
        interpreter.load(&[0x61, 0x42]);
        interpreter.cycle();
        assert_eq!(interpreter.v[1], 0x42);
        assert_eq!(interpreter.pc, 0x200 + 2);
    }

    #[test]
    fn test_7xnn() {
        // Test the 7XNN opcode (ADD VX, NN) to add a value to a register.
        let mut interpreter = Interpreter::default();
        interpreter.load(&[0x61, 0x42, 0x71, 0x0A]);
        interpreter.cycle();
        assert_eq!(interpreter.v[1], 0x42);

        interpreter.cycle();
        assert_eq!(interpreter.v[1], 0x4C);
        assert_eq!(interpreter.pc, 0x200 + 4);
    }

    #[test]
    fn test_sound_delay_and_play_sound() {
        let mut interpreter = Interpreter::default();

        interpreter.load(&[0x61, 0x03, 0xF1, 0x18]);
        interpreter.cycle();
        interpreter.cycle();

        // Execute the cycle for three iterations, which should decrement the sound_timer.
        for _ in 0..3 {
            interpreter.cycle();
        }

        // At this point, sound_timer should be 0.
        assert_eq!(interpreter.sound_timer, 0);
    }

    #[test]
    fn test_fx55_reg_dump() {
        let mut interpreter = Interpreter::default();

        // Initialize some values in registers
        interpreter.v[0] = 0x01;
        interpreter.v[1] = 0x02;
        interpreter.v[2] = 0x03;
        interpreter.v[3] = 0x04;

        interpreter.index = 0x300;

        // Load a program that uses REG_DUMP (FX55) to store the values of V0, V1, V2, and V3 in memory starting from the index register.
        interpreter.load(&[0xF3, 0x55]);

        // Execute the program
        interpreter.cycle();

        // Check if the values were stored in memory as expected
        assert_eq!(interpreter.memory[0x300], 0x01);
        assert_eq!(interpreter.memory[0x300 + 1], 0x02);
        assert_eq!(interpreter.memory[0x300 + 2], 0x03);
        assert_eq!(interpreter.memory[0x300 + 3], 0x04);

        // Make sure the program counter is incremented by 2 after the REG_DUMP instruction.
        assert_eq!(interpreter.pc, 0x200 + 2);
    }
}
