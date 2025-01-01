//! The panic handler

use crate::sbi::shutdown;
use core::panic::PanicInfo;
use log::*;
use crate::recevent::*;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    write_event(32);
    if let Some(location) = info.location() {
        error!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().as_str().unwrap()
        );
    } else {
        error!("[kernel] Panicked: {}", info.message().as_str().unwrap());
    }
    shutdown(true)
}
