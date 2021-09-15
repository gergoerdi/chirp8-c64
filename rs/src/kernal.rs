extern {
    pub fn k_setnam (fname: *const u8);
    pub fn k_setlfs (fd: u8, dev: u8, secondary: u8);
    pub fn k_open   () -> bool;
    pub fn k_chkin  (fd: u8);
    pub fn k_readst () -> u8;
    pub fn k_chrin  () -> u8;
    pub fn k_close  (fd: u8);
    pub fn k_clrchn ();
    pub fn k_ldplot (col: u8, row: u8);
    // pub fn k_load   (mode: u8, dest: **const u8) -> u8;
}

extern {
    pub fn __chrout (c: u8);
}
