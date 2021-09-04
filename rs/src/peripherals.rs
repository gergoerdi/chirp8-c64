use prelude::*;

pub const SCREEN_WIDTH : u8 = 64;
pub const SCREEN_HEIGHT : u8 = 32;

pub trait Peripherals {
    fn keep_running(&self) -> bool;

    fn set_pixel_row(&mut self, y: ScreenY, row: ScreenRow);
    fn get_pixel_row(&self, y: ScreenY) -> ScreenRow;
    fn redraw(&mut self);

    fn scan_key_row(&self, row: Byte) -> Byte;

    fn set_timer(&mut self, val: Byte);
    fn get_timer(&self) -> Byte;
    fn set_sound(&mut self, val: Byte);

    fn read_ram(&self, addr: Addr) -> Byte;
    fn write_ram(&mut self, addr: Addr, val: Byte);

    fn get_random(&mut self) -> Byte;
}
