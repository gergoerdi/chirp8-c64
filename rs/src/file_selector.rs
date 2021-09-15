use dir::*;
use kernal::*;

pub fn select_and_load_file (dest: *mut u8) {
    let mut n : u8 = 0;
    let mut dirents: [Dirent;144] = [Dirent{ d_name: [0;17]}; 144];

    // save border
    open_dir(2,8);
    while let Some(dirent) = read_dir() {
        dirents[n as usize] = dirent;
        n += 1;
    }
    close_dir(2);
    // restore border

    init_screen();
    let sel = select_file(&dirents, n);
    // set_istop_cb(flashBorder)
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
    2
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
