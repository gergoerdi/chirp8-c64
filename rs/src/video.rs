use chip8::prelude::*;
use chip8::peripherals::*;

static mut ROWS: [u64; 32] = [
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
    0x0008bef3cfa21000,
    0x0008884928321000,
    0x000f88492e2a1000,
    0x0008884928260000,
    0x0008bef3cfa21000,
    0x0000000000000000,
    0x0000008e22800000,
    0x0000008a3b800000,
    0x000000ae39000000,
    0x0000000000000000,
    0x000f08a8befbc000,
    0x000488ac88822000,
    0x00048aaa88e3c000,
    0x00048aa988828000,
    0x000f252888fa4000,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
];

#[no_mangle]
pub extern "C" fn clrText (scr: *mut u8) -> () {
    let arr = unsafe { core::slice::from_raw_parts_mut(scr, 1000) };

    for i in 0..1000 {
        arr[i] = 0x0f;
    }
}

const STRIDE : isize = 40;

fn draw_row (scr: *mut u8, y: ScreenY) -> () {
    let (_, rows, _) = unsafe { ROWS.align_to::<u8>() };
    let mut ptr = unsafe{ scr.offset((4 + (y / 2) as isize) * STRIDE + 4) };

    for i in 0..8 {
        let mut row1 = rows[(y as usize + 0) * 8 + (7 - i)];
        let mut row2 = rows[(y as usize + 1) * 8 + (7 - i)];

        for _ in 0..4 {
            // Take top two bits of row1 and row2
            let ch = (row1 >> 6) | ((row2 >> 6) << 2);
            row1 <<= 2;
            row2 <<= 2;
            unsafe { *ptr = ch; };
            ptr = unsafe{ ptr.offset(1) };
        }
    }
}

#[no_mangle]
pub extern "C" fn drawScreen (scr: *mut u8) -> () {
    for y in (0..32).step_by(2) {
        draw_row(scr, y)
    }
}
