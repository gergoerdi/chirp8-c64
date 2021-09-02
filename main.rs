#![no_std]

static mut ROWS: [u64; 32] = [
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
    0x0000000000000000
];

#[no_mangle]
pub extern "C" fn clrText (scr: *mut u8) -> () {
    for i in 0..1000 {
        unsafe { *(scr.offset(i)) = 0; }
    }
}

#[no_mangle]
pub extern "C" fn drawScreen (scr: *mut u8) -> () {
    let mut base = unsafe{ scr.offset(2 * 40 + 4) };
    for y in 0..16 { // TODO: this should be (0..32).step_by(2)
        let mut ptr = base;
        let mut row1 = unsafe { ROWS[y * 2] };
        let mut row2 = unsafe { ROWS[y * 2 + 1] };

        for x in 0..32 { // TODO: this should be (0..64).step_by(2)
            // Take top two bits of row1 and row2
            let ch : u8 = (((row1 >> 62) << 2) | (row2 >> 62)) as u8;
            row1 <<= 2;
            row2 <<= 2;
            unsafe { *ptr = ch; }
            ptr = unsafe{ ptr.offset(1) };
        }

        base = unsafe{ base.offset(40) };
    }
}
