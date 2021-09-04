use core::panic::PanicInfo;

#[panic_handler]
pub extern fn panic(_: &PanicInfo<'_>) -> ! {
    loop{}
}
