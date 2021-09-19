use chip8::prelude::*;

#[no_mangle]
pub extern "C" fn clear_screen (scr: *mut u8) {
    let arr = unsafe { core::slice::from_raw_parts_mut(scr, STRIDE * 25) };

    for i in 0 .. STRIDE * 25 {
        arr[i] = 0x0f;
    }

    let color_arr = unsafe { core::slice::from_raw_parts_mut(0xd800 as *mut u8, STRIDE * 25) };
    for i in 0 .. STRIDE * 25 {
        color_arr[i] = 0x0b;
    }
    for y in 4 .. 4 + 16 {
        for x in 4 .. 4 + 32 {
            color_arr[y * STRIDE + x] = 0x07;
        }
    }
}

const STRIDE : usize = 40;

fn draw_row (scr: &mut [u8], rows: &[u8], y: ScreenY) {
    let mut ptr = (4 + (y / 2) as usize) * STRIDE + 4;

    for i in 0..8 {
        let mut row1 = rows[(y as usize + 0) * 8 + (7 - i)];
        let mut row2 = rows[(y as usize + 1) * 8 + (7 - i)];

        for _ in 0..4 {
            // Take top two bits of row1 and row2
            let ch = (row1 >> 6) | ((row2 >> 6) << 2);
            row1 <<= 2;
            row2 <<= 2;
            scr[ptr] = ch;
            ptr += 1
        }
    }
}

pub fn draw_screen (scr: &mut [u8], rows: &[u64; 32]) {
    let (_, row_bytes, _) = unsafe { rows.align_to::<u8>() };
    for y in (0..32).step_by(2) {
        draw_row(scr, row_bytes, y)
    }
}
