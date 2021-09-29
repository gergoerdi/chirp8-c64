use kernal::*;

fn get_byte () -> Option<u8> {
    unsafe {
        // TODO: flash border

        if k_readst() != 0 {
            None
        } else {
            Some(k_chrin())
        }
    }
}

pub fn open_dir (fd: u8, dev: u8) {
    unsafe {
        k_setnam("$".as_ptr());
        k_setlfs(fd, dev, 0);
        k_open();
        k_chkin(fd);

        // Skip target address
        get_byte();
        get_byte();
    }

    // Skip first directory entry, since that's just the disk label
    read_dir();
}

pub fn close_dir (fd: u8) {
    unsafe {
        k_close(fd);
        k_clrchn();
    }
}

#[derive(Clone,Copy)]
pub struct Dirent {
    pub d_name: [u8; 16 + 1]
}

pub fn read_dir () -> Option<Dirent> {
    // Skip 2 bytes
    get_byte()?;
    get_byte()?;

    // BASIC line number
    get_byte()?;
    get_byte()?;

    // Skip to opening quote
    while get_byte()? != 0x22 {};

    let mut i : usize = 0;
    let mut s : [u8; 16 + 1] = [0; 16+1];

    loop {
        let c = get_byte()?;
        if c == 0x22 { break };
        s[i] = c;
        i += 1;
    }

    while get_byte()? != 0x00 {};

    Some(Dirent{
        d_name: s
    })
}
