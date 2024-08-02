use core::panic::PanicInfo;

use lock_api::RawMutex;
use vexide_core::io::Write;

use super::uart::uart1;
use crate::drivers::uart::UART1_DRIVER;

#[panic_handler]
fn panic_handler(info: &PanicInfo<'_>) -> ! {
    critical_section::with(|_| {
        unsafe {
            // SAFETY: The UART device will not be used after the panic has been printed
            // so we can consider the previous lock to be elapsed.
            UART1_DRIVER.raw().unlock();
        }
        writeln!(uart1(), "Kernel {}", info).unwrap();
        loop {}
    })
}
