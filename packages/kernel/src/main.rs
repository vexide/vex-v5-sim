#![no_std]
#![no_main]
#![feature(c_variadic)]
#![allow(static_mut_refs, clippy::empty_loop)]

extern crate alloc;

pub mod allocator;
pub mod hardware;
pub mod logger;
pub mod panic;
pub mod peripherals;
pub mod protocol;
pub mod sdk;
pub mod sync;
pub mod vectors;
pub mod xil;

use log::LevelFilter;
use logger::KernelLogger;
use peripherals::{GIC, PRIVATE_TIMER, UART1, WATCHDOG_TIMER};
use sdk::vexSystemLinkAddrGet;
use vex_v5_qemu_protocol::{code_signature::CodeSignature, HostBoundPacket};

use crate::{protocol::exit, sdk::{draw_error_box, vexTasksRun}};

extern "C" {
    /// Entrypoint of the user program. (located at 0x03800020)
    #[link_name = "_vex_startup"]
    fn vexStartup();

    /// Start address of user program memory.
    ///
    /// This is 32 bytes before vexStartup and contains the user code
    /// signature (header).
    #[link_name = "_user_memory_start"]
    static USER_MEMORY_START: *const ();

    /// Location of the exception vector table.
    #[link_name = "__vectors_start"]
    static VECTORS_START: *const ();
}

/// Kernel logging implementation.
///
/// This global implements the [`Log`] trait allowing us to to log information
/// to the host over a UART packet. See the `logger` module.
static LOGGER: KernelLogger = KernelLogger;

/// Kernel entrypoint.
///
/// This function is the Rust entrypoint of the kernel, and is called
/// immediately after the [`reset`] vector sets up the stack.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        // Set the vector base address register. This specifies to the CPU the start
        // address of the exception vector table, which is jumped to when the CPU
        // encounters an exception or interrupt. This is at address 0x10000 - the start
        // of kernel memory.
        //
        // The table is created in `vectors.s` and the handlers can be found in the
        // `vectors` module.
        vectors::set_vbar(core::ptr::addr_of!(VECTORS_START) as u32);

        // Register SDK exception handlers for data/prefetch/undefined aborts.
        vectors::register_abort_handlers();

        // Enable hardware floating-point instructions
        hardware::fpu::enable_vfp();

        // Enable MMU
        hardware::mmu::enable_mmu();

        // Enable IRQ and FIQ interrupts by masking CPSR with the IRQ and FIQ enable
        // bits.
        core::arch::asm!("cpsie if");

        // Initialize heap memory
        allocator::init_heap();
    }

    // Force-initialize all peripherals.
    //
    // If they fail to initialize, we want them to fail now rather than whenever
    // they're first accessed.
    GIC.force();
    PRIVATE_TIMER.force();
    WATCHDOG_TIMER.force();
    UART1.force();

    // Initialize UART kernel logger
    LOGGER.init(LevelFilter::Debug).unwrap();

    // Setup private timer peripheral and register a tick interrupt handler using
    // the GIC.
    //
    // This fires a timer interrupt every 1mS allowing us to keep track of system
    // time for [`vexSystemTimeGet`] as well for the purposes of ticking
    // FreeRTOS if needed.
    peripherals::setup_private_timer().unwrap();

    let code_header =
        unsafe { core::ptr::read(core::ptr::addr_of!(USER_MEMORY_START) as *const u32) };

    const ELF_HEADER: u32 = u32::from_le_bytes(*b"\x7FELF");
    if code_header == ELF_HEADER {
        draw_error_box([Some("Invalid user program !"), None, None]);
        log::error!("Note: This program appears to have an ELF header. `--program` only supports objcopied binaries, not ELFs.");
        exit(102);
    }
    let code_signature = CodeSignature::try_from(unsafe {
        core::ptr::read(core::ptr::addr_of!(USER_MEMORY_START) as *const vex_sdk::vcodesig)
    })
    .unwrap_or_else(|()| {
        draw_error_box([Some("Invalid user program !"), None, None]);
        exit(102);
    });

    // Send user code signature to host.
    log::debug!("Sending code signature to host.");
    protocol::send_packet(HostBoundPacket::CodeSignature(code_signature)).unwrap();
    log::debug!("Code signature sent successfully!");

    // Execute user program's entrypoint function.
    //
    // This is located 32 bytes after the code signature at 0x03800020.
    log::debug!(
        "Link address is {:#02x}. Calling user code.",
        vexSystemLinkAddrGet()
    );

    log::debug!("About to call vexTasksRun()...");
    unsafe {
        vexTasksRun(); // update devices once before running user code
    }
    log::debug!("vexTasksRun() completed, about to call vexStartup()...");

    // add a more obvious marker
    log::warn!("=== CALLING USER CODE (vexStartup) ===");
    unsafe {
        vexStartup();
    }
    log::warn!("=== USER CODE RETURNED (this should not happen) ===");

    unreachable!("User code should not return!");
}
