#![no_std]
#![no_main]

use log::info;

#[unsafe(no_mangle)]
extern "C" fn krnl_main() -> ! {
    api::serial::init();
    api::log::init();
    info!("Test build v{}", env!("CARGO_PKG_VERSION"));
    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let _ = api::log::print_panic(info);
    loop {}
}
