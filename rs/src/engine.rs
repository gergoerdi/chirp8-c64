use chip8::prelude::*;
use chip8::peripherals::*;
use chip8::machine::Machine;

use video::*;
use font::*;

// TODO: get length from file
const PROG : &[u8; 850] = include_bytes!("../../roms/hidden.ch8");

struct C64 {
    mem : [Byte; 4 * 1024 - 16 * 8],
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
        0
    }

    fn set_timer(&mut self, val: Byte) {
    }

    fn get_timer(&self) -> Byte {
        0
    }

    fn set_sound(&mut self, _val: Byte) {
    }

    fn read_ram(&self, addr: Addr) -> Byte {
        let idx = addr as usize;

        if idx < FONT_ROM.len() {
            FONT_ROM[idx]
        } else {
            self.mem[idx - FONT_ROM.len()]
        }
    }

    fn write_ram(&mut self, addr: Addr, val: Byte) {
        let idx = addr as usize;

        if idx >= FONT_ROM.len() {
            self.mem[idx - FONT_ROM.len()] = val;
        }
    }

    fn get_random(&mut self) -> Byte {
        0
    }
}

#[no_mangle]
pub extern "C" fn run (scr: *mut u8) {
    let mut c64 = C64{
        mem: [0;4 * 1024 - 16 * 8],
        scr: scr,
        vmem: [0;32]
    };

    let mut addr = 0x0200;
    for i in 0..PROG.len() {
        c64.write_ram(addr, PROG[i]);
        addr += 1;
    }

    clear_screen(c64.scr);
    let mut machine = Machine::new();
    loop {
        machine.step(&mut c64);
    }
}
