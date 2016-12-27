pub struct CPU {
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    stack: [u16; 16],
    delay_timer: u8,
    snd_timer: u8,
    sp: usize,
    pc: usize,
    opcode: u16
}

impl CPU {
    pub fn new() -> CPU {
        let mut cpu = CPU {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            stack: [0; 16],
            delay_timer: 0,
            snd_timer: 0,
            sp: 0,
            pc: 0x200,
            opcode: 0
        };
        for i in 0..80 {
            cpu.memory[i] = FONTS[i];
        }
        cpu
    }

    pub fn load_program(&mut self, data: Vec<u8>) {
        let mut i = 0x200;
        for b in 0..data.len() {
            self.memory[i] = data[b];
            i += 1;
        }
    }

    pub fn cycle(&mut self) {
        self.read_opcode();
        self.exec_opcode();

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.snd_timer > 0 {
            println!("**************************************BEEP**************************************");
            self.snd_timer -= 1;
        }

        println!("PC: {:04X} SP: {:02X} I: {:04X} DT: {:02X} ST:{:02X} OP: {:04X}",
            self.pc, self.sp, self.i, self.delay_timer, self.snd_timer, self.opcode);
        for i in 0..16 {
            print!("V{:X}: {:02X} ", i, self.v[i]);
        }
        println!();
        for i in 0..100000 {

        }
    }

    fn read_opcode(&mut self) {
        self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
    }

    fn exec_opcode(&mut self) {
        match self.opcode & 0xF000 {
            0x0000 => self.op0(),
            0x1000 => self.op1(),
            0x2000 => self.op2(),
            0x3000 => self.op3(),
            0x4000 => self.op4(),
            0x5000 => self.op5(),
            0x6000 => self.op6(),
            0x7000 => self.op7(),
            0x8000 => self.op8(),
            0x9000 => self.op9(),
            0xA000 => self.opa(),
            0xB000 => self.opb(),
            0xC000 => self.opc(),
            0xD000 => self.opd(),
            0xE000 => self.ope(),
            0xF000 => self.opf(),
            _ => ni(self.opcode, self.pc)
        }
    }

    fn op0(&mut self) {
        match self.opcode {
            0x00EE => self.pc = self.pop() as usize,
            _ => ni(self.opcode, self.pc)
        }
        self.pc += 2;
    }

    fn op1(&mut self) {
        let val = (self.opcode & 0x0FFF) as usize;
        self.pc = val;
    }

    fn op2(&mut self) {
        let val = (self.opcode & 0x0FFF) as usize;
        let pc = self.pc as u16;
        self.push(pc);
        self.pc = val;
    }

    fn op3(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let val = (self.opcode & 0x0FF) as u8;
        if self.v[x] == val {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn op4(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let val = (self.opcode & 0x00FF) as u8;
        if self.v[x] != val {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn op5(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;
        if self.v[x] == self.v[y] {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn op6(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let val = (self.opcode & 0x00FF) as u8;
        self.v[vx] = val;
        self.pc += 2;
    }

    // TODO: this instruction does not set carry flag in VF
    fn op7(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let val = (self.opcode & 0x0FF) as u8;
        self.v[x] += val;
        self.pc += 2;
    }

    fn op8(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;
        match self.opcode & 0x000F {
            0x0 => self.v[x] = self.v[y],
            0x1 => self.v[x] = self.v[x] | self.v[y],
            0x2 => self.v[x] = self.v[x] & self.v[y],
            0x3 => self.v[x] = self.v[x] ^ self.v[y],
            0x4 => {
                let (result, overflow) = self.v[x].overflowing_add(self.v[y]);
                self.v[x] = result;
                self.v[0xF] = if overflow { 1 } else { 0 };
            },
            0x5 => {
                let (result, overflow) = self.v[x].overflowing_sub(self.v[y]);
                self.v[x] = result;
                self.v[0xF] = if overflow { 0 } else { 1 };
            },
            0x6 => {
                let lsb = match self.v[x] & 0x01 {
                    0x00 => false,
                    0x01 => true,
                    _ => panic!(),
                };
                self.v[0xF] = if lsb { 1 } else { 0 };
                self.v[x] >> 1;
            },
            0x7 => {
                let (result, overflow) = self.v[y].overflowing_sub(self.v[x]);
                self.v[x] = result;
                self.v[0xF] = if overflow { 0 } else { 1 };
            },
            0xE => {
                let msb = match self.v[x] & 0x80 {
                    0x00 => false,
                    0x80 => true,
                    _ => panic!(),
                };
                self.v[0xF] = if msb { 1 } else { 0 };
                self.v[x] << 1;
            },
            _ => ni(self.opcode, self.pc)
        }
        self.pc += 2;
    }

    fn op9(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;
        match self.opcode & 0x000F {
            0x0 => {
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            },
            _ => ni(self.opcode, self.pc)
        }
        self.pc += 2;
    }

    fn opa(&mut self) {
        let val  = self.opcode & 0x0FFF;
        self.i |= val;
        self.pc += 2;
    }


    fn opb(&mut self) {
        ni(self.opcode, self.pc);
        self.pc += 2;
    }

    fn opc(&mut self) {
        ni(self.opcode, self.pc);
        self.pc += 2;
    }

    fn opd(&mut self) {
        ni(self.opcode, self.pc);
        self.pc += 2;
    }

    fn ope(&mut self) {
        ni(self.opcode, self.pc);
        self.pc += 2;
    }

    // TODO: implement remaining codes
    fn opf(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        match self.opcode & 0x00FF {
            0x07 => self.v[x] = self.delay_timer,
            0x15 => self.delay_timer = self.v[x],
            0x18 => self.snd_timer = self.v[x],
            0x1E => self.i = self.i + self.v[x] as u16,
            0x29 => self.i = self.v[x] as u16 * 5,
            0x55 => {
                let i = self.i & 0x0FFF;
                for vx in 0..x+1 {
                    self.memory[(i as usize + vx)] = self.v[vx as usize];
                }
            },
            0x65 => {
                let mut i = (self.i & 0x0FFF) as usize;
                for vx in 0..x+1 {
                    self.v[vx] = self.memory[i];
                    i += 1;
                }
            },
            _ => ni(self.opcode, self.pc)
        }
        self.pc += 2;
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp]
    }
}

fn ni(opcode: u16, pc: usize) {
    println!("*******************************************************");
    println!("Not implemented: PC: {:04X} Opcode: {:04X}", pc, opcode);
    println!("*******************************************************");
}

const FONTS: [u8; 80] = [
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
0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];
