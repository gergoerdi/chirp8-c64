#[no_mangle]
pub extern "C" fn make_charset() {
    let charset = unsafe { core::slice::from_raw_parts_mut(0xc000 as *mut u8, 256 * 8) };

    for c in 0..16 {
        for y in 0..4 {
            charset[c * 8 + y] =
                (if c & 0x1 != 0 { 0x0f } else { 0x00 }) |
                (if c & 0x2 != 0 { 0xf0 } else { 0x00 });
        }
        for y in 4..8 {
            charset[c * 8 + y] =
                (if c & 0x4 != 0 { 0x0f } else { 0x00 }) |
                (if c & 0x8 != 0 { 0xf0 } else { 0x00 });
        }
    }
}
