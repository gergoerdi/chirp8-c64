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

struct C64<'a> {
    mem : &'a mut [u8],
    vmem : [u64; 32],
    scr : &'a mut [u8]
}

impl Peripherals for C64<'_> {
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

    fn get_keys(&self) -> u16 {
        let code = get_key();
        let key = match code {
            0x38 => Some(0x1),
            0x3b => Some(0x2),
            0x08 => Some(0x3),
            0x0b => Some(0xc),

            0x3e => Some(0x4),
            0x09 => Some(0x5),
            0x0e => Some(0x6),
            0x11 => Some(0xd),

            0x0a => Some(0x7),
            0x0d => Some(0x8),
            0x12 => Some(0x9),
            0x15 => Some(0xe),

            0x0c => Some(0xa),
            0x17 => Some(0x0),
            0x14 => Some(0xb),
            0x1f => Some(0xf),

            _    => None
        };

        match key {
            Some(key) => { 1 << key }
            _ => { 0 }
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
        let idx = addr as usize;

        if idx < FONT_ROM.len() {
            FONT_ROM[idx]
        } else if idx < 0x200 {
            0
        } else {
            self.mem[idx - 0x200]
        }
    }

    fn write_ram(&mut self, addr: Addr, val: Byte) {
        let idx = addr as usize;

        if idx >= 0x200 {
            self.mem[idx - 0x200] = val;
        }
    }

    fn get_random(&mut self) -> Byte {
        0
    }
}

#[no_mangle]
pub extern "C" fn run (mem: *mut u8, scr: *mut u8) {
    let mut c64 = C64{
        mem: unsafe { core::slice::from_raw_parts_mut(mem, RAMSIZE) },
        scr: unsafe { core::slice::from_raw_parts_mut(scr, 40 * 25) },
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
