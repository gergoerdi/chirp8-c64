use dir::*;
use kernal::*;
use machine::get_key;

#[no_mangle]
pub extern "C" fn select_and_load_file (dest: *mut u8) {
    let mut n : u8 = 0;
    let mut dirents: [Dirent;144] = [Dirent{ d_name: [0;17]}; 144];

    // TODO: save border
    open_dir(2,8);
    while let Some(dirent) = read_dir() {
        dirents[n as usize] = dirent;
        n += 1;
    }
    close_dir(2);
    // TODO: restore border

    init_screen();
    let sel = select_file(&dirents, n);
    let mut dest_end = dest;
    load(8, dirents[sel as usize].d_name.as_ptr(), &mut dest_end);
}

const WINDOW_HEIGHT : u8 = 12;
const WINDOW_Y0 : u8 = 6;
const WINDOW_WIDTH : u8 = 16;
const WINDOW_X0 : u8 = 11;

fn init_screen() {
    unsafe {
        __chrout(0x93); // Clear screen

        // Top border
        k_ldplot(WINDOW_X0, WINDOW_Y0);
        __chrout(0x75);
        for _ in 0..16 {
            __chrout(0x60);
        }
        __chrout(0xae);

        // Bottom border
        k_ldplot(WINDOW_X0, WINDOW_Y0 + WINDOW_HEIGHT + 1);
        __chrout(0xad);
        for _ in 0..16 {
            __chrout(0x60); // 0x64 also looks quite nice here
        }
        __chrout(0x6b);

        // Left and right border
        for y in WINDOW_Y0 + 1 .. WINDOW_Y0 + WINDOW_HEIGHT + 1 {
            k_ldplot(WINDOW_X0, y);
            __chrout(0x62);
            k_ldplot(WINDOW_X0 + WINDOW_WIDTH + 1, y);
            __chrout(0x62);
        }
    }
}

fn select_file(dirents: &[Dirent], n: u8) -> u8 {
    let mut sel : u8 = 0;
    let mut offset : u8 = 0;
    let mut wait_release = false;

    loop {
        for i in 0 .. WINDOW_HEIGHT {
            if offset + i >= n { continue };
            let dirent = &dirents[(offset + i) as usize];

            unsafe {
                let current = offset + i == sel;
                k_ldplot(WINDOW_X0 + 1, WINDOW_Y0 + 1 + i);
                if current { __chrout(0x12); } // reverse on
                for c in dirent.d_name {
                    if c == 0 { break; }
                    __chrout(c);
                }
                if current { __chrout(0x92); } // reverse off
            }
        }

        if wait_release { while get_key() != 0x40 {} }

        loop {
            match get_key() {
                // Z     X      C      V
                0x0c | 0x17 | 0x14 | 0x14 if sel < n - 1 => {
                    wait_release = true;
                    sel += 1;
                    if sel - offset > WINDOW_HEIGHT { offset += 1 }
                    break;
                }
                // Q     W      E      R
                0x3e | 0x09 | 0x0e | 0x11 if sel > 0 => {
                    wait_release = true;
                    sel -= 1;
                    if sel < offset { offset -= 1 }
                    break;
                }

                // Enter
                0x01 => { return sel }
                _ => {}
            }
        }
    }
}

pub fn load(dev: u8, fname: *const u8, dest: &mut *mut u8) -> u8 {
    unsafe {
        start_flash();
        k_setnam(fname);
        k_setlfs(1, dev, 0);
        let result = k_load(0, dest);
        end_flash();
        return result
    }
}
