use core::ptr::{read_volatile};

use chip8::prelude::*;
use chip8::peripherals::*;
use chip8::cpu::CPU;

use video::*;
use font::*;

extern "C" {
    static mut timer_reg: u8;
}

const RAMSIZE : usize = 4 * 1024 - 0x200;

struct C64 {
    mem : *mut Byte,
    vmem : [u64; 32],
    scr : *mut u8
}

impl Peripherals for C64 {
    fn keep_running(&self) -> bool {
        true
    }

    fn set_pixel_row(&mut self, y: ScreenY, row: ScreenRow) {
        self.vmem[y as usize] = row;
    }

    fn get_pixel_row(&self, y: ScreenY) -> ScreenRow {
        self.vmem[y as usize]
    }

    fn redraw(&mut self) {
        draw_screen(self.scr, &self.vmem);
    }

    fn scan_key_row(&self, row: Byte) -> Byte {
        let code = get_key();
        match row {
            0 => match code {
                0x38 => 0b0001,
                0x3b => 0b0010,
                0x08 => 0b0100,
                0x0b => 0b1000,
                _    => 0b0000
            },
            1 => match code {
                0x3e => 0b0001,
                0x09 => 0b0010,
                0x0e => 0b0100,
                0x11 => 0b1000,
                _    => 0b0000
            },
            2 => match code {
                0x0a => 0b0001,
                0x0d => 0b0010,
                0x12 => 0b0100,
                0x15 => 0b1000,
                _    => 0b0000
            },
            3 => match code {
                0x0c => 0b0001,
                0x17 => 0b0010,
                0x14 => 0b0100,
                0x1f => 0b1000,
                _    => 0b0000
            },
            _ => unreachable!()
        }
    }

    fn set_timer(&mut self, val: Byte) {
        unsafe{ timer_reg = val }
    }

    fn get_timer(&self) -> Byte {
        unsafe{ timer_reg }
    }

    fn set_sound(&mut self, _val: Byte) {
    }

    fn read_ram(&self, addr: Addr) -> Byte {
        let mem = unsafe { core::slice::from_raw_parts(self.mem, RAMSIZE) };
        let idx = addr as usize;

        if idx < FONT_ROM.len() {
            FONT_ROM[idx]
        } else if idx < 0x200 {
            0
        } else {
            mem[idx - 0x200]
        }
    }

    fn write_ram(&mut self, addr: Addr, val: Byte) {
        let mem = unsafe { core::slice::from_raw_parts_mut(self.mem, RAMSIZE) };
        let idx = addr as usize;

        if idx >= 0x200 {
            mem[idx - 0x200] = val;
        }
    }

    fn get_random(&mut self) -> Byte {
        0
    }
}

#[no_mangle]
pub extern "C" fn run (mem: *mut u8, scr: *mut u8) {
    let mut c64 = C64{
        mem: mem,
        scr: scr,
        vmem: [0;32]
    };

    let mut cpu = CPU::new();
    loop {
        cpu.step(&mut c64);
    }
}

pub fn get_key () -> u8 {
    unsafe { read_volatile(0x00c5 as *const u8) }
}
