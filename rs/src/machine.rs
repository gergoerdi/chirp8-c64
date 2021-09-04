pub use prelude::*;
pub use opcodes::*;
pub use peripherals::Peripherals;

pub struct Machine {
    regs: [Byte; 16],
    addr: Addr,
    pc: Addr,
    stack: [Addr; 16],
    sp: usize,
}

impl Machine {
    pub fn new() -> Machine {
        Machine{ regs : [0; 16],
                 addr: 0,
                 pc: 0x200,
                 stack: [0; 16],
                 sp: 0
        }
    }

    fn eval(&self, arg: Arg) -> Byte {
        match arg {
            Arg::Reg(vx) => self.regs[vx as usize],
            Arg::Imm(nn) => nn
        }
    }

    fn arith(op: Arith, x: Byte, y: Byte) -> (Byte, Option<bool>) {
        match op {
            Arith::Load => (y, None),
            Arith::Or => (x | y, None),
            Arith::And => (x & y, None),
            Arith::XOr => (x ^ y, None),
            Arith::Add => {
                let (z, f) = u8::overflowing_add(x, y);
                (z, Some(f))
            },
            Arith::Sub => {
                let (z, f) = u8::overflowing_sub(x, y);
                (z, Some(f))
            },
            Arith::SubFlip => {
                let (z, f) = u8::overflowing_sub(y, x);
                (z, Some(f))
            },
            Arith::ShiftL => (x << 1, Some(x & 0x80 != 0)),
            Arith::ShiftR => (x >> 1, Some(x & 0x01 != 0))
        }
    }

    fn set_flag(&mut self, flag: bool) {
        self.regs[0xf] = if flag { 1 } else { 0 };
    }

    fn key_coords(key: Byte) -> (Byte, Byte) {
        match key & 0xf {

            0x1 => (0, 0),
            0x2 => (0, 1),
            0x3 => (0, 2),
            0xc => (0, 3),

            0x4 => (1, 0),
            0x5 => (1, 1),
            0x6 => (1, 2),
            0xd => (1, 3),

            0x7 => (2, 0),
            0x8 => (2, 1),
            0x9 => (2, 2),
            0xe => (2, 3),

            0xa => (3, 0),
            0x0 => (3, 1),
            0xb => (3, 2),
            0xf => (3, 3),

            _ => unreachable!()
        }
    }

    fn coords_key(coords: (Byte, Byte)) -> Byte {
        match coords {
            (0, 0) => 0x1,
            (0, 1) => 0x2,
            (0, 2) => 0x3,
            (0, 3) => 0xc,

            (1, 0) => 0x4,
            (1, 1) => 0x5,
            (1, 2) => 0x6,
            (1, 3) => 0xd,

            (2, 0) => 0x7,
            (2, 1) => 0x8,
            (2, 2) => 0x9,
            (2, 3) => 0xe,

            (3, 0) => 0xa,
            (3, 1) => 0x0,
            (3, 2) => 0xb,
            (3, 3) => 0xf,

            _ => unreachable!()
        }
    }

    fn wait_key<P>(&self, io: &mut P) -> (Byte, Byte) where P: Peripherals {
        let mut init_states = [0; 4];
        for (row, state) in init_states.iter_mut().enumerate() {
            *state = io.scan_key_row(row as Byte)
        }

        while io.keep_running() {
            for (row, old_state) in init_states.iter_mut().enumerate() {
                let new_state = io.scan_key_row(row as Byte);
                let mut fresh_keys = new_state & !*old_state;
                if fresh_keys != 0 {
                    let mut col = 0;
                    loop {
                        if fresh_keys & 0x1 != 0 {
                            return (row as Byte, col)
                        }
                        fresh_keys >>= 1;
                        col += 1;
                    }
                }

                *old_state &= new_state;
            }
        };

        (0, 0)
    }

    pub fn step<P>(&mut self, io: &mut P) where P: Peripherals {
        let hi = io.read_ram(self.pc); self.pc += 1;
        let lo = io.read_ram(self.pc); self.pc += 1;

        // match decode(hi, lo) {
        //     None => println!("0x{:04x} 0x{:02x} 0x{:02x}", self.pc-2, hi, lo),
        //     Some(op) => println!("0x{:04x} {:?}", self.pc-2, op)
        // }

        match decode(hi, lo).unwrap() {
            Op::Sys(_addr) => {
                // TODO
            },
            Op::Call(addr) => {
                self.stack[self.sp] = self.pc;
                self.sp = (self.sp + 1) & 0x0f;
                self.pc = addr;
            },
            Op::Ret => {
                self.sp = (self.sp - 1) & 0x0f;
                self.pc = self.stack[self.sp];
            },
            Op::Jump(addr) => {
                self.pc = addr;
            },
            Op::Skip(when, vx, target) => {
                let x = self.regs[vx as usize];
                let y = self.eval(target);
                let skip = match when {
                    Cmp::Eq => x == y,
                    Cmp::NEq => x != y
                };
                if skip {
                    self.pc += 2;
                }
            },
            Op::LoadImm(vx, imm) => {
                self.regs[vx as usize] = imm
            },
            Op::AddImm(vx, imm) => {
                self.regs[vx as usize] = u8::wrapping_add(self.regs[vx as usize], imm)
            },
            Op::Arith(op, vx, vy) => {
                let x = self.regs[vx as usize];
                let y = self.regs[vy as usize];
                let (z, flag) = Machine::arith(op, x, y);
                self.regs[vx as usize] = z;
                flag.map(|flag| { self.set_flag(flag); });
            },
            Op::LoadI(addr) => {
                self.addr = addr;
            },
            Op::AddI(vx) => {
                let addr = self.addr + self.regs[vx as usize] as u16;
                self.set_flag(addr > 0x0fff);
                self.addr = addr & 0x0fff;
            },
            Op::GetTimer(vx) => {
               self.regs[vx as usize] = io.get_timer();
            },
            Op::SetTimer(vx) => {
                io.set_timer(self.regs[vx as usize]);
            },
            Op::JumpV0(addr) => {
                self.pc = addr + self.regs[0] as Addr;
            },
            Op::Random(vx, mask) => {
                let rnd = io.get_random();
                self.regs[vx as usize] = rnd & mask;
            },
            Op::Hex(vx) => {
                self.addr = (self.regs[vx as usize] as u16 & 0x0f) << 3;
            },
            Op::StoreBCD(vx) => {
                let x = self.regs[vx as usize];
                io.write_ram(self.addr, x / 100);
                io.write_ram(self.addr + 1, (x % 100) / 10);
                io.write_ram(self.addr + 2, x % 10);
            },
            Op::Save(vx) => {
                for i in 0..vx as usize +1 {
                    io.write_ram(self.addr + i as Addr, self.regs[i])
                }
            },
            Op::Restore(vx) => {
                for i in 0..vx as usize +1 {
                    self.regs[i] = io.read_ram(self.addr + i as Addr)
                }
            },
            Op::Draw(vx, vy, n) => {
                let mut collision = false;
                let xd = (self.regs[vx as usize]) & 0x3f;
                for i in 0..n {
                    let yd = (self.regs[vy as usize] + i) & 0x1f;
                    let dat = io.read_ram(self.addr + i as Addr);
                    let row = ((dat as ScreenRow) << 56) >> xd;

                    let old_row = io.get_pixel_row(yd);
                    let new_row = old_row ^ row;
                    collision |= old_row & row != 0;
                    io.set_pixel_row(yd, new_row);
                };
                io.redraw();
                self.set_flag(collision);
            },
            Op::ClearScr => {
                for y in 0..32 {
                    io.set_pixel_row(y, 0);
                }
            },
            Op::SkipKey(cond, vx) => {
                let (row, col) = Machine::key_coords(self.regs[vx as usize]);
                let pressed = (io.scan_key_row(row) & (1 << col)) != 0;
                let target = match cond {
                    Cmp::Eq => true,
                    Cmp::NEq => false
                };
                if pressed == target {
                    self.pc += 2;
                }
            },
            Op::WaitKey(vx) => {
                self.regs[vx as usize] = Machine::coords_key(self.wait_key(io));
            },
            Op::SetSound(vx) => {
                io.set_sound(self.regs[vx as usize]);
            },
        }
    }
}
