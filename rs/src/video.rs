use prelude::*;

pub fn clear_screen (scr: *mut u8) {
    let arr = unsafe { core::slice::from_raw_parts_mut(scr, 1000) };

    for i in 0..1000 {
        arr[i] = 0x0f;
    }
}

const STRIDE : isize = 40;

fn draw_row (scr: *mut u8, rows: &[u8], y: ScreenY) {
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

pub fn draw_screen (scr: *mut u8, rows: &[u64; 32]) {
    let (_, row_bytes, _) = unsafe { rows.align_to::<u8>() };
    for y in (0..32).step_by(2) {
        draw_row(scr, row_bytes, y)
    }
}
