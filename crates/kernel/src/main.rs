#![no_std]
#![no_main]

#[unsafe(no_mangle)]
extern "C" fn krnl_main() -> ! {
    api::serial::init();
    api::log::init();

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
