use chip8::prelude::*;

pub const FONT_ROM : [Byte; 16 * 8] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x00, 0x00, 0x00,
    0x20, 0x60, 0x20, 0x20, 0x70, 0x00, 0x00, 0x00,
    0xF0, 0x10, 0xF0, 0x80, 0xF0, 0x00, 0x00, 0x00,
    0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x00, 0x00, 0x00,
    0x90, 0x90, 0xF0, 0x10, 0x10, 0x00, 0x00, 0x00,
    0xF0, 0x80, 0xF0, 0x10, 0xF0, 0x00, 0x00, 0x00,
    0xF0, 0x80, 0xF0, 0x90, 0xF0, 0x00, 0x00, 0x00,
    0xF0, 0x10, 0x20, 0x40, 0x40, 0x00, 0x00, 0x00,
    0xF0, 0x90, 0xF0, 0x90, 0xF0, 0x00, 0x00, 0x00,
    0xF0, 0x90, 0xF0, 0x10, 0xF0, 0x00, 0x00, 0x00,
    0xF0, 0x90, 0xF0, 0x90, 0x90, 0x00, 0x00, 0x00,
    0xE0, 0x90, 0xE0, 0x90, 0xE0, 0x00, 0x00, 0x00,
    0xF0, 0x80, 0x80, 0x80, 0xF0, 0x00, 0x00, 0x00,
    0xE0, 0x90, 0x90, 0x90, 0xE0, 0x00, 0x00, 0x00,
    0xF0, 0x80, 0xF0, 0x80, 0xF0, 0x00, 0x00, 0x00,
    0xF0, 0x80, 0xF0, 0x80, 0x80, 0x00, 0x00, 0x00
];
