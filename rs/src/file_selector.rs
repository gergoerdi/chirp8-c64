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

fn init_screen() {
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
