//! VEXos System Functions

use core::{arch::asm, ffi::c_void, sync::atomic::Ordering};

use log::{error, info};
use vex_sdk::*;

use crate::{
    timer_interrupt_handler,
    utils::exit,
    xil::{
        gic::{
            XScuGic_Connect, XScuGic_LookupConfig, XScuGic_SetPriorityTriggerType,
            XPAR_SCUGIC_0_DIST_BASEADDR, XSCUGIC_MAX_NUM_INTR_INPUTS,
        },
        timer::{
            XScuTimer, XScuTimer_ClearInterruptStatus, XScuTimer_EnableInterrupt,
            XScuTimer_IsExpired, XScuTimer_Start, XScuTimer_Stop, XPAR_SCUTIMER_INTR,
        },
        wdt::{
            XScuWdt_CfgInitialize, XScuWdt_GetControlReg, XScuWdt_LoadWdt, XScuWdt_LookupConfig,
            XScuWdt_SetControlReg, XScuWdt_SetTimerMode, XScuWdt_Start, XPAR_XSCUWDT_0_BASEADDR,
        },
    },
    INTERRUPT_CONTROLLER, PRIVATE_TIMER, SYSTEM_TIME, WATCHDOG_TIMER,
};

pub fn vexPrivateApiDisable(sig: u32) {}
pub fn vexStdlibMismatchError(param_1: u32, param_2: u32) {}
pub fn vexScratchMemoryPtr(ptr: *mut *mut core::ffi::c_void) -> i32 {
    Default::default()
}
pub fn vexScratchMemoryLock() -> bool {
    Default::default()
}
pub fn vexScratchMemoryUnock() {}
pub fn vexSystemTimeGet() -> u32 {
    SYSTEM_TIME.load(Ordering::Acquire)
}
pub fn vexGettime() -> time {
    Default::default()
}
pub fn vexGetdate() -> date {
    Default::default()
}
pub fn vexSystemMemoryDump() {}
pub fn vexSystemDigitalIO(pin: u32, value: u32) {}
pub fn vexSystemStartupOptions() -> u32 {
    Default::default()
}
pub fn vexSystemExitRequest() {
    exit(0);
}
pub fn vexSystemHighResTimeGet() -> u64 {
    const PERIPH_BASE_ADDR: u32 = 0xF8F00000;

    const GLOBAL_TIMER_BASE_ADDRESS: u32 = PERIPH_BASE_ADDR + 0x0200;

    const GLOBAL_TIMER_COUNTER_REGISTER_LOW: u32 = GLOBAL_TIMER_BASE_ADDRESS /* + 0x00 */;
    const GLOBAL_TIMER_COUNTER_REGISTER_HIGH: u32 = GLOBAL_TIMER_BASE_ADDRESS + 0x04;

    const CLOCKS_PER_USEC: u64 = 666666687 / (2 * 1000000);

    let mut low: u32;
    let mut high: u32;

    loop {
        unsafe {
            high = core::ptr::read_volatile(GLOBAL_TIMER_COUNTER_REGISTER_HIGH as *const u32);
            low = core::ptr::read_volatile(GLOBAL_TIMER_COUNTER_REGISTER_LOW as *const u32);

            if core::ptr::read_volatile(GLOBAL_TIMER_COUNTER_REGISTER_HIGH as *const u32) == high {
                break;
            }
        }
    }

    (((high as u64) << 32_u32) | (low as u64)) / CLOCKS_PER_USEC
}
pub fn vexSystemPowerupTimeGet() -> u64 {
    // powerup time is the same as execution time in our case
    vexSystemHighResTimeGet()
}
pub fn vexSystemLinkAddrGet() -> u32 {
    // no multi-file support yet
    0x0
}
pub fn vexSystemTimerGet(param_1: u32) -> u32 {
    Default::default()
}
pub fn vexSystemUsbStatus() -> u32 {
    Default::default()
}
pub fn vexSystemTimerStop() {
    unsafe {
        XScuTimer_Stop(PRIVATE_TIMER.get_mut());
    }
}

/// Clears the timer interrupt status if the timer has expired.
pub fn vexSystemTimerClearInterrupt() {
    // Realistically I think this should be a call XScuTimer_ClearInterruptStatus
    // and NOT the timer interrupt handler (which also increments the time for vexSystemTimeGet),
    // but this is supposedly the behavior observed in VEXos, so we'll do it too.
    unsafe { timer_interrupt_handler(PRIVATE_TIMER.get_mut() as *mut XScuTimer as _) }
}

/// Reinitializes the timer interrupt with a given tick handler and priority for the private timer instance.
pub fn vexSystemTimerReinitForRtos(
    priority: u32,
    handler: extern "C" fn(data: *mut c_void),
) -> i32 {
    unsafe {
        let gic = INTERRUPT_CONTROLLER.get_mut();
        let timer = PRIVATE_TIMER.get_mut();

        // Set tick interrupt priority
        // PROS sets this to the lowest possible priority (portLOWEST_USABLE_INTERRUPT_PRIORITY << portPRIORITY_SHIFT).
        XScuGic_SetPriorityTriggerType(
            gic,
            XPAR_SCUTIMER_INTR,
            priority as u8,
            3, // Rising-edge trigger
        );

        // Install tick handler
        let status = XScuGic_Connect(
            gic,
            XPAR_SCUTIMER_INTR,
            Some(handler),
            timer as *mut XScuTimer as _,
        );

        // Restart the timer and enable the timer interrupt
        if status == 0 {
            XScuTimer_Start(timer);

            // Clear interrupt status if the timer expired.
            if XScuTimer_IsExpired(timer) {
                XScuTimer_ClearInterruptStatus(timer);
            }

            // Enable the timer interrupt.
            XScuTimer_EnableInterrupt(timer);

            return 0;
        }
    }

    1
}

/// Handles an IRQ using the interrupt controller's handler table.
pub fn vexSystemApplicationIRQHandler(ulICCIAR: u32) {
    // The ID of the interrupt is obtained by bitwise anding the ICCIAR value
    let interrupt_id = ulICCIAR & 0x3FF;

    unsafe {
        // Re-enable interrupts.
        asm!("cpsie i");

        // Check for a valid interrupt ID.
        if interrupt_id < (XSCUGIC_MAX_NUM_INTR_INPUTS as u32) {
            // Call respective interrupt handler from the vector table.
            let cfg = XScuGic_LookupConfig(XPAR_SCUGIC_0_DIST_BASEADDR as *mut u32);
            let handler_entry = (*cfg).HandlerTable[interrupt_id as usize];
            (handler_entry.handler).unwrap()(handler_entry.callback_ref);
        }
    }
}

/// Initializes the CPU1 watchdog timer.
pub fn vexSystemWatchdogReinitRtos() -> i32 {
    unsafe {
        let wdt = WATCHDOG_TIMER.get_mut();

        let config = XScuWdt_LookupConfig(XPAR_XSCUWDT_0_BASEADDR as *mut u32);
        let status = XScuWdt_CfgInitialize(wdt, config, (*config).BaseAddr);

        if status != 0 {
            // XScuWdt_CfgInitialize returned XST_DEVICE_IS_STARTED, meaning that the watchdog timer was already started.
            // If the watchdog was already initialized previously, there's no need to go further, so we'll indicate to the
            // caller that this happened by returning 1.
            return 1;
        }

        // Configure control register
        let mut control_reg = XScuWdt_GetControlReg(wdt);
        control_reg |= 0xff << 0x08;
        XScuWdt_SetControlReg(wdt, control_reg);

        // Load timer and start.
        XScuWdt_LoadWdt(wdt, u32::MAX);
        XScuWdt_SetTimerMode(wdt);
        XScuWdt_Start(wdt);
    }

    0
}
pub fn vexSystemWatchdogGet() -> u32 {
    Default::default()
}
pub fn vexSystemBoot() {}

pub fn vexSystemUndefinedException() {
    error!("Undefined Instruction Exception");
}
pub fn vexSystemFIQInterrupt() {
    info!("FIQ");
}
pub fn vexSystemIQRQnterrupt() {
    info!("IRQ");
}
pub fn vexSystemSWInterrupt() {
    info!("Software Interrupt");
}
pub fn vexSystemDataAbortInterrupt() {
    error!("Data Abort Exception");
}
pub fn vexSystemPrefetchAbortInterrupt() {
    error!("Prefetch Abort Exception");
}
